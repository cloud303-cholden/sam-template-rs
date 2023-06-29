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

#[derive(Serialize)]
struct Response {
    status_code: i32,
    body: String,
}

async fn function_handler(_event: LambdaEvent<_>) -> Result<Response, Box<dyn std::error::Error>> {
    let config = ::aws_config::load_from_env().await;

    let resp = Response {
        status_code: 200,
        body: "OK".to_string(),
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env_filter = env::var("ENV_FILTER")
        .unwrap_or("aws_config=warn,hyper=info".to_string());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    run(service_fn(function_handler)).await
}
