use secrecy::ExposeSecret;
use secrecy::Secret;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Clone)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl std::convert::TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!("{} is not a supported environment.", other)),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub url: Option<String>,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        if let Some(url) = &self.url {
            url.parse::<PgConnectOptions>().expect("Failed to parse DATABASE_URL")
        } else {
            PgConnectOptions::new()
                .host(&self.host)
                .username(&self.username)
                .password(self.password.expose_secret())
                .port(self.port)
                .ssl_mode(PgSslMode::Require)
        }
    }

    pub fn with_db(&self) -> PgConnectOptions {
        if let Some(url) = &self.url {
            url.parse::<PgConnectOptions>().expect("Failed to parse DATABASE_URL")
        } else {
            self.without_db().database(&self.database_name)
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Load base configuration
    let mut builder = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")).required(true));

    // Load environment-specific configuration
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    builder = builder.add_source(
        config::File::from(configuration_directory.join(format!("{}.yaml", environment.as_str())))
            .required(true),
    );

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    builder = builder.add_source(config::Environment::with_prefix("app").separator("__"));

    // Add in settings from environment variables without prefix, using '_' as separator
    // E.g. `DATABASE_URL` would set `database.url`
    builder = builder.add_source(config::Environment::default().separator("_"));

    let settings = builder.build()?;
    settings.try_deserialize()
}
