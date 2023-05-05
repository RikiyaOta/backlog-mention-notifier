use crate::account_mapping::{get_backlog_users, AppConfig};
use lambda_http::{Request, RequestPayloadExt};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct BacklogProject {
    projectKey: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogUser {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogNotification {
    user: BacklogUser,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogComment {
    id: u32,
    content: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogCommentAddedContent {
    key_id: u32,
    summary: String,
    comment: BacklogComment,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogIssueRelatedWebhookPayload {
    id: u32,
    project: BacklogProject,
    r#type: u8,
    content: BacklogCommentAddedContent,
    notifications: Vec<BacklogNotification>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommentedIssue {
    project_key: String,
    issue_key: u32,
    issue_subject: String,
    pub notified_backlog_user_names: Vec<String>,
    comment: String,
}

fn is_commented_event(payload: &BacklogIssueRelatedWebhookPayload) -> bool {
    payload.r#type == 3
}

/*
 * 注意：コメントの文章の中に、ただの文字としてメンションの情報が入っている。
 * また、ユーザー名の後に空白等が入っているとも限らない。"@RikiyaOtaホゲ" もありえる。
 * なので、あらかじめユーザー名を保持しておいて、その情報をもとに Regex を組み立ててサーチする必要がある。
 */
fn extract_backlog_user_names(
    comment: &String,
    backlog_user_name_candidates: Vec<String>,
) -> Vec<String> {
    backlog_user_name_candidates
        .iter()
        .filter(|backlog_user_name| {
            let regex = Regex::new(&format!("@{}", backlog_user_name)).unwrap();
            regex.is_match(&comment)
        })
        .map(|s| s.to_string())
        .collect()
}

pub fn parse_webhook_payload(
    event: &Request,
    app_config: &AppConfig,
) -> Result<CommentedIssue, String> {
    match event.payload::<BacklogIssueRelatedWebhookPayload>() {
        Ok(payload) => match payload {
            None => Err("Payload is None.".to_string()),
            Some(payload) => {
                if is_commented_event(&payload) {
                    let comment = payload.content.comment.content;
                    Ok(CommentedIssue {
                        project_key: payload.project.projectKey,
                        issue_key: payload.content.key_id,
                        issue_subject: payload.content.summary,
                        notified_backlog_user_names: extract_backlog_user_names(
                            &comment,
                            get_backlog_users(&app_config),
                        ),
                        comment,
                    })
                } else {
                    Err("This payload is not commented-event.".to_string())
                }
            }
        },
        Err(error) => Err(error.to_string()),
    }
}
