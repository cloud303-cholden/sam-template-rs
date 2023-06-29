use lambda_http::{
    Body,
    Response,
}
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

async fn function_handler(
    _event: LambdaEvent<_>,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    // Load AWS config from environment.
    let config = ::aws_config::load_from_env().await;

    // Default to an HTTP response.
    Ok(Response::builder()
        .status(200)
        .body("ok".into())?
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Filter out config cache warning and hyper debug insanity.
    let env_filter = env::var("ENV_FILTER")
        .unwrap_or("aws_config=warn,hyper=info".to_string());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    run(service_fn(function_handler)).await
}
