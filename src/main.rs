use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, RequestPayloadExt, Response};
use serde::{Deserialize, Serialize};

// https://crates.io/crates/lambda_runtime
// https://github.com/cargo-lambda/cargo-lambda/issues/397

#[derive(Deserialize, Serialize, Debug)]
struct BacklogProject {
    projectKey: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogComment {
    id: u32,
    content: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogCommentAddedContent {
    key_id: u32,
    comment: BacklogComment,
}

#[derive(Deserialize, Serialize, Debug)]
struct BacklogCommentAddedWebhookPayload {
    id: u32,
    project: BacklogProject,
    r#type: u8,
    content: BacklogCommentAddedContent
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
    match event.payload::<BacklogCommentAddedWebhookPayload>() {
        Ok(payload) => match payload {
            None => println!("Payload is None!!!"),
            Some(payload) => println!("Payload is {:?}", payload),
        },
        Err(error) => {
            println!("PayloadError is {:?}", error);
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
