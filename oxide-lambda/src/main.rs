use log::info;
use std::time::Duration;
use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_config::meta::region::RegionProviderChain;
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

/// Save the reliability curve to S3
async fn save_reliability(
    client: &Client,
    bucket: &str,
    reliability: &[f64],
    filename: &str,
    expires_in: u64,
) -> Result<(), Error> {
    let expires_in = Duration::from_secs(expires_in);
    let serialized_data = bincode::serialize(reliability)?;

    let presigned_request = client
        .put_object()
        .bucket(bucket)
        .key(filename)
        .body(serialized_data.into())
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(())
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    info!("Handling a reliability request...");
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let bucket = std::env::var("BUCKET_NAME")
        .expect("A BUCKET_NAME must be set in this app's Lambda environment variables.");

    info!("Draw reliability curve from Weibull");
    let shape = event.payload.weibull_shape;
    let scale = event.payload.weibull_scale;
    let reliability = draw_reliability(shape, scale);
    let num_points = reliability.len();
    let filename = event.payload.asset_id;

    save_reliability(&client, &bucket, &reliability, &filename, 900).await?;
    info!("Saved reliability to S3");

    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Asset reliability curve has {} timesteps!", num_points),
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
