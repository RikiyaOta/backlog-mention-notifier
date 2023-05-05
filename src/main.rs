pub mod account_mapping;
pub mod backlog_webhook_parser;
pub mod slack_api;

use account_mapping::AppConfig;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

fn get_notified_slack_user_id(
    app_config: &AppConfig,
    notified_backlog_user_names: &Vec<String>,
) -> Vec<String> {
    notified_backlog_user_names
        .iter()
        .filter_map(|backlog_user_name| {
            account_mapping::map_to_slack_user_id(&app_config, &backlog_user_name)
        })
        .collect()
}

// TODO:
// webhook に対しては即座に200を返して、別スレッドで処理をするようにしたい。
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let app_config = account_mapping::get_app_config();

    match backlog_webhook_parser::parse_webhook_payload(&event, &app_config) {
        Err(error_message) => println!("Error: {}", error_message),
        Ok(commented_issue) => {
            for slack_user_id in get_notified_slack_user_id(
                &app_config,
                &commented_issue.notified_backlog_user_names,
            )
            .iter()
            {
                println!(
                    "Posting messages to slack... slack_user_id={}",
                    slack_user_id
                );
                match slack_api::post_direct_message(&app_config, slack_user_id, &commented_issue)
                    .await
                {
                    Ok(response) => {
                        println!("Finish sending!, response={}", response);
                    }
                    Err(error) => println!(
                        "Failed posting a message. slack_user_id={}, error={}",
                        slack_user_id, error
                    ),
                }
            }
        }
    }

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
