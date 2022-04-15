extern crate envconfig;
use serde::Deserialize;
use envconfig::Envconfig;

#[derive(Envconfig, Deserialize)]
struct AppConfig {
    #[envconfig(from = "APP_HOST")]
    pub url: String,
    #[envconfig(from = "APP_PORT")]
    pub port: u16,
}

#[derive(Envconfig, Deserialize)]
struct DaoConfig {
    #[envconfig(from = "DB_CONNECTION_STRING")]
    pub db_conn: String,
}

#[derive(Deserialize)]
pub struct Config {
    app: AppConfig,
    dao: DaoConfig,
}

impl Config {
    pub fn new() -> Self {
        let app_config = AppConfig::init_from_env().unwrap();
        let dao_config = DaoConfig::init_from_env().unwrap();
        return Self {app: app_config, dao: dao_config};
    }

    pub fn get_app_url(&self) -> String {
        return format!("{0}:{1}", self.app.url, self.app.port);
    }

    pub fn get_database_url(&self) -> String {
        return format!("{}", self.dao.db_conn);
    }
}
