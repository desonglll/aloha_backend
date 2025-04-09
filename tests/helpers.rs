use aloha_backend::configuration::{get_configuration, DatabaseSettings};
use aloha_backend::dto::query::DtoQuery;
use aloha_backend::dto::response::DtoResponse;
use aloha_backend::models::group_permission::GroupPermissionResponse;
use aloha_backend::models::permission::PermissionResponse;
use aloha_backend::models::user::UserResponse;
use aloha_backend::models::user_group::UserGroupResponse;
use aloha_backend::models::user_permission::UserPermissionResponse;
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
    pub async fn post_user_group(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<UserGroupResponse> {
        self.api_client
            .post(format!("{}/user_groups", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroupResponse>()
            .await
    }
    pub async fn get_all_user_groups(
        &self,
    ) -> reqwest::Result<DtoResponse<Vec<UserGroupResponse>>> {
        self.api_client
            .get(format!("{}/user_groups", self.address))
            .query(&DtoQuery::default_query())
            .send()
            .await?
            .json::<DtoResponse<Vec<UserGroupResponse>>>()
            .await
    }
    pub async fn get_user_group_by_id(&self, id: Uuid) -> reqwest::Result<UserGroupResponse> {
        self.api_client
            .get(format!("{}/user_groups/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroupResponse>()
            .await
    }
    pub async fn put_user_group(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<UserGroupResponse> {
        self.api_client
            .put(format!("{}/user_groups", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroupResponse>()
            .await
    }
    pub async fn delete_user_group(&self, id: Uuid) -> reqwest::Result<UserGroupResponse> {
        self.api_client
            .delete(format!("{}/user_groups/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserGroupResponse>()
            .await
    }

    pub async fn post_user(&self, body: &serde_json::Value) -> reqwest::Result<UserResponse> {
        self.api_client
            .post(format!("{}/users", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserResponse>()
            .await
    }

    pub async fn get_all_users(&self) -> reqwest::Result<DtoResponse<Vec<UserResponse>>> {
        self.api_client
            .get(format!("{}/users", self.address))
            .query(&DtoQuery::default_query())
            .send()
            .await?
            .json::<DtoResponse<Vec<UserResponse>>>()
            .await
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> reqwest::Result<UserResponse> {
        self.api_client
            .get(format!("{}/users/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserResponse>()
            .await
    }
    pub async fn delete_users(&self, ids: &[Uuid]) -> reqwest::Result<Vec<UserResponse>> {
        self.api_client
            .delete(format!("{}/users", self.address))
            .header("Content-Type", "application/json")
            .json(ids)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Vec<UserResponse>>()
            .await
    }

    pub async fn put_user(&self, body: &serde_json::Value) -> reqwest::Result<UserResponse> {
        self.api_client
            .put(format!("{}/users", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserResponse>()
            .await
    }

    pub async fn delete_user(&self, id: Uuid) -> reqwest::Result<UserResponse> {
        self.api_client
            .delete(format!("{}/users/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserResponse>()
            .await
    }

    pub async fn post_permission(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<PermissionResponse> {
        self.api_client
            .post(format!("{}/permissions", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<PermissionResponse>()
            .await
    }

    pub async fn get_all_permissions(
        &self,
    ) -> reqwest::Result<DtoResponse<Vec<PermissionResponse>>> {
        self.api_client
            .get(format!("{}/permissions", self.address))
            .query(&DtoQuery::default_query())
            .send()
            .await?
            .json::<DtoResponse<Vec<PermissionResponse>>>()
            .await
    }

    pub async fn get_permission_by_id(&self, id: Uuid) -> reqwest::Result<PermissionResponse> {
        self.api_client
            .get(format!("{}/permissions/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<PermissionResponse>()
            .await
    }

    pub async fn put_permission(
        &self,
        id: Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Result<PermissionResponse> {
        self.api_client
            .put(format!("{}/permissions/{}", self.address, id))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<PermissionResponse>()
            .await
    }

    pub async fn delete_permission(&self, id: Uuid) -> reqwest::Result<PermissionResponse> {
        self.api_client
            .delete(format!("{}/permissions/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<PermissionResponse>()
            .await
    }

    pub async fn post_group_permission(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<GroupPermissionResponse> {
        self.api_client
            .post(format!("{}/group_permissions", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<GroupPermissionResponse>()
            .await
    }

    pub async fn get_all_group_permissions(
        &self,
    ) -> reqwest::Result<DtoResponse<Vec<GroupPermissionResponse>>> {
        self.api_client
            .get(format!("{}/group_permissions", self.address))
            .query(&DtoQuery::default_query())
            .send()
            .await?
            .json::<DtoResponse<Vec<GroupPermissionResponse>>>()
            .await
    }

    pub async fn get_group_permissions_by_group_id(
        &self,
        group_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<GroupPermissionResponse>>> {
        self.api_client
            .get(format!(
                "{}/group_permissions/group/{}",
                self.address, group_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<GroupPermissionResponse>>>()
            .await
    }

    pub async fn get_group_permissions_by_permission_id(
        &self,
        permission_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<GroupPermissionResponse>>> {
        self.api_client
            .get(format!(
                "{}/group_permissions/permission/{}",
                self.address, permission_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<GroupPermissionResponse>>>()
            .await
    }

    pub async fn delete_group_permission(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<GroupPermissionResponse> {
        self.api_client
            .delete(format!("{}/group_permissions", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<GroupPermissionResponse>()
            .await
    }

    pub async fn delete_group_permissions_by_group_id(
        &self,
        group_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<GroupPermissionResponse>>> {
        self.api_client
            .delete(format!(
                "{}/group_permissions/group/{}",
                self.address, group_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<GroupPermissionResponse>>>()
            .await
    }

    pub async fn delete_group_permissions_by_permission_id(
        &self,
        permission_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<GroupPermissionResponse>>> {
        self.api_client
            .delete(format!(
                "{}/group_permissions/permission/{}",
                self.address, permission_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<GroupPermissionResponse>>>()
            .await
    }

    pub async fn post_user_permission(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<UserPermissionResponse> {
        self.api_client
            .post(format!("{}/user_permissions", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserPermissionResponse>()
            .await
    }

    pub async fn get_all_user_permissions(
        &self,
    ) -> reqwest::Result<DtoResponse<Vec<UserPermissionResponse>>> {
        self.api_client
            .get(format!("{}/user_permissions", self.address))
            .query(&DtoQuery::default_query())
            .send()
            .await?
            .json::<DtoResponse<Vec<UserPermissionResponse>>>()
            .await
    }

    pub async fn get_user_permissions_by_user_id(
        &self,
        user_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<UserPermissionResponse>>> {
        self.api_client
            .get(format!(
                "{}/user_permissions/user/{}",
                self.address, user_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<UserPermissionResponse>>>()
            .await
    }

    pub async fn get_user_permissions_by_permission_id(
        &self,
        permission_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<UserPermissionResponse>>> {
        self.api_client
            .get(format!(
                "{}/user_permissions/permission/{}",
                self.address, permission_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<UserPermissionResponse>>>()
            .await
    }

    pub async fn delete_user_permission(
        &self,
        body: &serde_json::Value,
    ) -> reqwest::Result<UserPermissionResponse> {
        self.api_client
            .delete(format!("{}/user_permissions", self.address))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<UserPermissionResponse>()
            .await
    }

    pub async fn delete_user_permissions_by_user_id(
        &self,
        user_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<UserPermissionResponse>>> {
        self.api_client
            .delete(format!(
                "{}/user_permissions/user/{}",
                self.address, user_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<UserPermissionResponse>>>()
            .await
    }

    pub async fn delete_user_permissions_by_permission_id(
        &self,
        permission_id: Uuid,
    ) -> reqwest::Result<DtoResponse<Vec<UserPermissionResponse>>> {
        self.api_client
            .delete(format!(
                "{}/user_permissions/permission/{}",
                self.address, permission_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<DtoResponse<Vec<UserPermissionResponse>>>()
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
    let endpoint = application.endpoint();
    let address = format!("http://127.0.0.1:{}/{}", application_port, endpoint);

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

fn is_port_open(port: u16) -> bool {
    TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
