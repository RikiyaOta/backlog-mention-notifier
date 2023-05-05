pub mod account_mapping;

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, RequestPayloadExt, Response};
use regex::Regex;
use serde::{Deserialize, Serialize};

// https://crates.io/crates/lambda_runtime
// https://github.com/cargo-lambda/cargo-lambda/issues/397

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
struct CommentedIssue {
    project_key: String,
    issue_key: u32,
    issue_subject: String,
    // TODO: ↓何が扱いやすいかな。ID？
    notified_backlog_users: Vec<String>,
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
fn extract_backlog_user_names(comment: &String) -> Vec<String> {
    let backlog_user_names = vec!["RikiyaOta", "Test1", "Test2"];

    backlog_user_names
        .iter()
        .filter(|backlog_user_name| {
            let regex = Regex::new(&format!("@{}", backlog_user_name)).unwrap();
            regex.is_match(&comment)
        })
        .map(|s| s.to_string())
        .collect()
}

fn parse_webhook_payload(event: &Request) -> Result<CommentedIssue, String> {
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
                        // TODO: Parse notified users.
                        notified_backlog_users: extract_backlog_user_names(&comment),
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

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // TODO:
    // 1. Parse Request Body.
    // 2. Judge webhook event type.
    // 3. Get comment content.
    // 4. Extract mentioned users.
    // 5. Decide the users' slack account id.
    // 6. Post DMs to the users.

    match parse_webhook_payload(&event) {
        Ok(payload) => println!("Payload: {:?}", payload),
        Err(error_message) => println!("Error: {}", error_message),
    }
    
    // DEBUG:
    let app_config = account_mapping::get_app_config();
    println!("App Config: {:?}", app_config);

    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
