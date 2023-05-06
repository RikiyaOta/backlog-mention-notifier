use crate::app_config::{AppConfig, UserAccountMapping};

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
