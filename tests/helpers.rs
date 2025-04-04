use aloha_backend::configuration::{get_configuration, DatabaseSettings};
use aloha_backend::models::user_group::UserGroup;
use aloha_backend::startup::{get_connection_pool, Application};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpStream;
use tracing::info;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let _default_filter_level = "info".to_string();
});
#[derive(Debug)]
pub struct TestUser {
    id: Uuid,
    pub username: String,
    pub password: String,
}
impl TestUser {
    pub fn generate() -> Self {
        Self {
            id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: "admin".into(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut OsRng);
        // Match production parameters
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
        dbg!(&password_hash);
        sqlx::query!(
            "INSERT INTO users (id, username, password_hash)
            VALUES ($1, $2, $3)",
            self.id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }
}
#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
    pub(crate) test_user: TestUser,
    pub api_client: reqwest::Client,
}
impl TestApp {
    pub async fn post_user_group(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(format!("{}/user_group", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_user_group_by_id(&self, id: Uuid) -> reqwest::Result<UserGroup> {
        self.api_client
            .get(format!("{}/user_group/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroup>()
            .await
    }
    pub async fn put_user_group(&self, body: &serde_json::Value) -> reqwest::Result<UserGroup> {
        self.api_client
            .put(format!("{}/user_group", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroup>()
            .await
    }
    pub async fn delete_user_group(&self, id: Uuid) -> reqwest::Result<UserGroup> {
        self.api_client
            .delete(format!("{}/user_group/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroup>()
            .await
    }
}
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };
    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application_port);
    dbg!(&address);

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .cookie_store(true)
        .build()
        .unwrap();
    let test_app = TestApp {
        address,
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        api_client: client,
        test_user: TestUser::generate(),
    };
    test_app.test_user.store(&test_app.db_pool).await;
    info!(
        "Is port {} open? {}",
        test_app.port,
        is_port_open(test_app.port)
    );

    test_app
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
fn is_port_open(port: u16) -> bool {
    TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok()
}
