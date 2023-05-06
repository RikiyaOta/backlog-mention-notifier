use crate::app_config::AppConfig;
use crate::backlog_webhook_parser::CommentedIssue;
use reqwest;
use serde::Serialize;
use tracing::debug;

#[derive(Debug, Serialize)]
struct SlackMessageBlockSectionText {
    r#type: String,
    text: String,
}

impl SlackMessageBlockSectionText {
    fn new(r#type: String, text: String) -> Self {
        Self { r#type, text }
    }
}

#[derive(Debug, Serialize)]
struct SlackMessageBlockSection {
    r#type: String,
    text: SlackMessageBlockSectionText,
}

impl SlackMessageBlockSection {
    fn new(text_type: String, text: String) -> Self {
        Self {
            r#type: "section".to_string(),
            text: SlackMessageBlockSectionText::new(text_type, text),
        }
    }
}

#[derive(Debug, Serialize)]
struct SlackMessageBlockDivider {
    r#type: String,
}

impl SlackMessageBlockDivider {
    fn new() -> Self {
        Self {
            r#type: "divider".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum SlackMessageBlock {
    Section(SlackMessageBlockSection),
    Divider(SlackMessageBlockDivider),
}

#[derive(Debug, Serialize)]
struct SlackPostMessageBody {
    channel: String,
    blocks: Vec<SlackMessageBlock>,
}

fn build_request_body(
    app_config: &AppConfig,
    slack_user_id: &String,
    commented_issue: &CommentedIssue,
) -> SlackPostMessageBody {
    let blocks = vec![
        SlackMessageBlock::Section(SlackMessageBlockSection::new(
            "plain_text".to_string(),
            format!(
                ":memo: {} さんが課題にコメントしました。",
                commented_issue.comment_creator
            ),
        )),
        SlackMessageBlock::Divider(SlackMessageBlockDivider::new()),
        SlackMessageBlock::Section(SlackMessageBlockSection::new(
            "mrkdwn".to_string(),
            format!(
                "*<https://{backlog_space_id}.backlog.com/view/{project_key}-{issue_key}#comment-{comment_id}|{project_key}-{issue_key} {issue_subject}>*",
                backlog_space_id = app_config.backlog_space_id,
                project_key = commented_issue.project_key,
                issue_key = commented_issue.issue_key,
                comment_id = commented_issue.comment_id,
                issue_subject = commented_issue.issue_subject
            ),
        )),
        SlackMessageBlock::Section(
            SlackMessageBlockSection::new(
                "mrkdwn".to_string(),
                commented_issue.comment_content.to_string()
            )
        )
    ];

    SlackPostMessageBody {
        channel: slack_user_id.to_string(),
        blocks,
    }
}

pub async fn post_direct_message(
    app_config: &AppConfig,
    slack_user_id: &String,
    commented_issue: &CommentedIssue,
) -> Result<String, String> {
    let body = build_request_body(app_config, slack_user_id, commented_issue);
    debug!("Request Body: {:?}", body);
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
