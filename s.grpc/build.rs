use std::{env, path::PathBuf};

fn main() {
    // protoc_rust_grpc::Codegen::new()
    //     .out_dir("./src/protos")
    //     .input("./protos/helloworld.proto")
    //     .rust_protobuf(true)
    //     .run()
    //     .expect("error compiling protocol buffer");

    let proto_file = "./protos/helloworld.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/protos")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
