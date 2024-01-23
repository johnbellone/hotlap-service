// SPDX-License-Identifier: Apache-2.0
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/hotlap_service.proto")?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("hotlap_service_descriptor.bin"))
        .compile(&["../proto/hotlap_service.proto"], &["../proto"])
        .unwrap();
    Ok(())
}
