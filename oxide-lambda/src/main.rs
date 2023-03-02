use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use statrs::distribution::Weibull;
use oxide::weibull::reliability;

#[derive(Deserialize)]
struct Request {
    asset_id: String,
    weibull_shape: f64,
    weibull_scale: f64,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// Draw a survival function for a Weibull distribution
fn draw_reliability(shape: f64, scale: f64) -> Vec<f64> {
    let weibull = Weibull::new(shape, scale).unwrap();
    let reliability = reliability(weibull, 1_000_000);
    reliability
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let shape = event.payload.weibull_shape;
    let scale = event.payload.weibull_scale;
    let reliability = draw_reliability(shape, scale);

    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Asset reliability curve has {} timesteps!", reliability.len()),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
