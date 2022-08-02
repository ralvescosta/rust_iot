## grpc

- **Requirements:**

  - apt install -y protobuf-compiler
   
  - config cargo/bins in your HOME

  - cargo install protobuf-codegen


protoc --rust_out ./src/protos protos/helloworld.proto