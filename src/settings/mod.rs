use std::env;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub secret: String,
    pub issuer: String,
    pub expiry: i64,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub filter: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub auth: Auth,
    pub log: Log,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        //s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("APP_ENV").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
