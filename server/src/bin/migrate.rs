// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use dotenvy::dotenv;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "hotlap-service-migrate")]
#[command(bin_name = "hotlap-service-migrate")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _args: Args = Args::parse();

    Ok(())
}
