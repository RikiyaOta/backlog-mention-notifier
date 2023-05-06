use config::{Config, FileFormat};
use core::panic;
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
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| String::from("development"));

    println!("app_env={}", app_env);

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

    println!(
        "Slack bot OAuth token: {}",
        app_config.slack_bot_oauth_token
    );
    println!(
        "User account mapping: {:?}",
        app_config.user_account_mapping
    );

    app_config
}

pub fn get_backlog_users(app_config: &AppConfig) -> Vec<String> {
    app_config
        .user_account_mapping
        .iter()
        .map(|mapping| (&mapping.backlog_user_name).to_string())
        .collect()
}

pub fn map_to_slack_user_id(app_config: &AppConfig, backlog_user_name: &String) -> Option<String> {
    let user_account_mapping = &app_config.user_account_mapping;

    match user_account_mapping
        .iter()
        .find(|mapping| &mapping.backlog_user_name == backlog_user_name)
    {
        Some(UserAccountMapping { slack_user_id, .. }) => Some(slack_user_id.to_string()),
        None => None,
    }
}
