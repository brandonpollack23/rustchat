fn main() {
  let proto_root = "proto";
  println!("cargo:rerun-if-changed={}", proto_root);
  protoc_grpcio::compile_grpc_protos(
    &["messenger.proto"],
    &[proto_root],
    "src/proto"
  ).expect("Failed to compile gRPC protos");
}