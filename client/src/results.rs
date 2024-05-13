// SPDX-License-Identifier: Apache-2.0

use std::io::{BufRead, BufReader};
use tokio::fs::File;

async fn read_results(file: File) -> Result<usize, std::io::Error> {
    let file = file.into_std().await;
    tokio::task::spawn_blocking(move || {}).await?;
}
