use crate::routes::Routes;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub routes: Routes,
    pub redis_uri: SecretString,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}
#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: SecretString,
    pub endpoint: String,
}
pub enum Environment {
    Local,
    Production,
}
impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);
        options
            .log_statements(tracing::log::LevelFilter::Trace)
            .to_owned()
    }
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a support environment. Use either `local` or `production`",
                other
            )),
        }
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("ALOHA_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse ALOHA_ENVIRONMENT");
    let environment_filename = format!("{}.toml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ))
        .add_source(config::File::from(
            configuration_directory.join("route.toml"),
        ))
        .add_source(
            config::Environment::with_prefix("ALOHA")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use crate::configuration::{get_configuration, ApplicationSettings};
    use secrecy::{ExposeSecret, SecretString};

    #[test]
    fn test_secret_string() {
        let secret_string: SecretString = SecretString::from("test");
        assert_eq!(secret_string.expose_secret(), "test");
    }

    #[test]
    fn test_application_settings_deserialize() {
        let port = 8080;
        let host = String::from("127.0.0.1");
        let base_url = String::from("http://127.0.0.1");
        let hmac_secret = SecretString::from("hello");
        let endpoint = String::from("/api");
        let application_settings: ApplicationSettings = ApplicationSettings {
            port,
            host: host.clone(),
            base_url: base_url.clone(),
            hmac_secret,
            endpoint,
        };

        assert_eq!(application_settings.port, port);
        assert_eq!(application_settings.host, host);
        assert_eq!(application_settings.base_url, base_url);
        assert_eq!(
            application_settings.hmac_secret.expose_secret(),
            &String::from("hello")
        );
    }

    #[test]
    fn test_get_configuration() {
        let settings = get_configuration().unwrap();
        assert_eq!(settings.application.host, "127.0.0.1");
        // assert_eq!(settings.application.port, 0);
    }
}
