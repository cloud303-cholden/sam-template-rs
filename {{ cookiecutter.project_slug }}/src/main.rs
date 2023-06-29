// Remove this.
#![allow(unused_imports)]

use lambda_runtime::{
    run,
    service_fn,
    Error,
    LambdaEvent,
};
use serde::{
    Serialize,
    Deserialize,
};
use tracing::info;

// Prefer to replace this struct with a concrete LambdaEvent.
#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    status_code: i32,
    body: String,
}

async fn function_handler(
    _event: LambdaEvent<Request>,
) -> Result<Response, Box<dyn std::error::Error>> {
    // Load AWS config from environment.
    let _config = ::aws_config::load_from_env().await;

    // Default to an HTTP response.
    Ok(Response {
        status_code: 200,
        body: "ok".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Filter out config cache warning and hyper debug insanity.
    let env_filter = std::env::var("ENV_FILTER")
        .unwrap_or("aws_config=warn,hyper=info".to_string());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    run(service_fn(function_handler)).await
}
