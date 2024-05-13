// SPDX-License-Identifier: Apache-2.0
use clap::Parser;
use hotlap_service_client::TokenInterceptor;
use hotlap_service_sdk::pb::{driver_service_client::DriverServiceClient, DriverRequest};
use tonic::transport::Channel;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "hotlap-service-client")]
#[command(bin_name = "hotlap-service-client")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[clap(short = 'a', long = "address", default_value = "[::1]:50051")]
    address: std::net::SocketAddr,

    #[clap(value_parser = humantime::parse_duration, default_value = "5s")]
    timeout: std::time::Duration,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    let channel = Channel::from_static("http://[::1]:50051")
        .timeout(args.timeout)
        .rate_limit(5, std::time::Duration::from_secs(5))
        .concurrency_limit(256)
        .connect()
        .await?;

    // TODO: Implement JWT authentication/authorization here.
    let interceptor = TokenInterceptor::default();
    let mut client = DriverServiceClient::with_interceptor(channel, interceptor);

    // TODO: https://github.com/duckdb/duckdb-rs/blob/main/examples/appender.rs

    let req = DriverRequest::default();
    debug!("req: {:?}", req);

    let res = client.read(req).await?;
    tracing::debug!("res: {:?}", res);

    Ok(())
}
