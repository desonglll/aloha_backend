use crate::api_doc::ApiDoc;
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::api_routes;
use utoipa::OpenApi;

use crate::routes::health_check::health_check;

use actix_cors::Cors;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;
use utoipa_swagger_ui::SwaggerUi;

pub struct ApplicationBaseUrl(pub String);
#[derive(Clone)]
pub struct HmacSecret(pub SecretString);

pub struct Application {
    port: u16,
    server: Server,
    endpoint: String,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port,
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
        )
        .await?;
        Ok(Self {
            port,
            server,
            endpoint: configuration.application.endpoint,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn endpoint(&self) -> String {
        self.endpoint.clone()
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        let port = self.port;
        info!("Server starting on port {}", port);
        let result = self.server.await;
        info!("Server on port {} has stopped", port);
        result
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    println!("Connecting to DB with: {:?}", configuration.with_db());
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/api/health_check", web::get().to(health_check))
            .configure(api_routes)
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
