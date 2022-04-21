extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
                                   .out_dir("src/protos")
                                   .inputs(&["src/protos/models.proto"])
                                   .include("src/protos")
                                   .run()
                                   .expect("Protobuf codegen failed.");
}