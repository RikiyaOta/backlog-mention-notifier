pub mod account_mapping;
pub mod backlog_webhook_parser;

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let app_config = account_mapping::get_app_config();

    match backlog_webhook_parser::parse_webhook_payload(&event, &app_config) {
        Err(error_message) => println!("Error: {}", error_message),
        Ok(commented_issue) => {
            commented_issue
                .notified_backlog_user_names
                .iter()
                .for_each(|backlog_user_name| {
                    let maybe_slack_user_id =
                        account_mapping::map_to_slack_user_id(&app_config, &backlog_user_name);
                    println!(
                        "backlog_user_name: {}, slack_user_id: {:?}",
                        backlog_user_name, maybe_slack_user_id
                    );

                    // 後はこの　Slack User ID と Bot OAuth Token を使ってメッセージを投げるだけ！
                });
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
