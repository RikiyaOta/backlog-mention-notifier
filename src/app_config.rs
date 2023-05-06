use config::{Config, FileFormat};
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct UserAccountMapping {
    pub backlog_user_name: String,
    pub slack_user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub slack_bot_oauth_token: String,
    pub user_account_mapping: Vec<UserAccountMapping>,
    pub backlog_space_id: String,
}

impl From<config::Config> for AppConfig {
    fn from(value: config::Config) -> Self {
        Self {
            slack_bot_oauth_token: value.get("slack_bot_oauth_token").unwrap(),
            user_account_mapping: value.get("user_account_mapping").unwrap(),
            backlog_space_id: value.get("backlog_space_id").unwrap(),
        }
    }
}

pub fn get_app_config() -> AppConfig {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| String::from("dev"));

    let config_filename = format!("config/config.{}.json", app_env);

    let config_str = match fs::read_to_string(&config_filename) {
        Ok(config) => config,
        Err(error) => panic!("Cannot read the app configuration! error={:?}", error),
    };

    let settings = match Config::builder()
        .add_source(config::File::from_str(&config_str, FileFormat::Json))
        .build()
    {
        Ok(settings) => settings,
        Err(error) => panic!("Cannot build the app configuration! error={:?}", error),
    };

    let app_config = match TryInto::<AppConfig>::try_into(settings) {
        Ok(app_config) => app_config,
        Err(error) => panic!("Cannot convert to app config! error={:?}", error),
    };

    app_config
}
