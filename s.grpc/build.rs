fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("./src/protos")
        .input("./protos/helloworld.proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
}
