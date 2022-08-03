use std::{env, path::PathBuf};

fn main() {
    let proto_file = "./protos/iot.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/protos")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
