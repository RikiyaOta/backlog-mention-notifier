use crate::{account_mapping::AppConfig, backlog_webhook_parser::CommentedIssue};
use reqwest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SlackPostMessageBody {
    channel: String,
    // TODO: もっと見栄え良く！！！
    text: String,
}

fn build_request_body(
    slack_user_id: &String,
    commented_issue: &CommentedIssue,
) -> SlackPostMessageBody {
    SlackPostMessageBody {
        channel: slack_user_id.to_string(),
        text: format!(
            r#"title: {}
        url: https://example.backlog.com/views/{}-{}
        comment: {}
        "#,
            commented_issue.issue_subject,
            commented_issue.project_key,
            commented_issue.issue_key,
            commented_issue.comment
        ),
    }
}

pub async fn post_direct_message(
    app_config: &AppConfig,
    slack_user_id: &String,
    commented_issue: &CommentedIssue,
) -> Result<String, String> {
    let body = build_request_body(slack_user_id, commented_issue);
    let client = reqwest::Client::new();

    let result = client
        .post("https://slack.com/api/chat.postMessage")
        .header(
            "Authorization",
            format!("Bearer {}", app_config.slack_bot_oauth_token),
        )
        .json(&body)
        .send()
        .await;

    match result {
        Ok(response) => Ok(response.text().await.unwrap()),
        Err(error) => Err(error.to_string()),
    }
}
