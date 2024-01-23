// SPDX-License-Identifier: Apache-2.0
use clap::Parser;
use pb::{hotlap_service_client::HotlapServiceClient, DatumRequest};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

pub mod pb {
    tonic::include_proto!("hotlap_service");
}

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

fn datum_request() -> DatumRequest {
    return DatumRequest {
        uuid: uuid::Uuid::now_v7().to_string(),
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    let channel = Channel::from_static("http://[::1]:50051")
        .timeout(args.timeout)
        .rate_limit(5, std::time::Duration::from_secs(5))
        .concurrency_limit(256)
        .connect()
        .await?;

    // TODO: Implement JWT authentication/authorization here.
    let token: MetadataValue<_> = "Bearer deadbeef".parse()?;
    let mut client = HotlapServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let requests = vec![datum_request(), datum_request(), datum_request()];

    let _ = client.record(tokio_stream::iter(requests)).await?;

    Ok(())
}
