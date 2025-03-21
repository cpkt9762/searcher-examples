use tonic_build::configure;

fn main() {
    configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Enable proto3 optional fields
        .compile_protos(
            // Use the new method name
            &[
                "protos/auth.proto",
                "protos/block.proto",
                "protos/block_engine.proto",
                "protos/bundle.proto",
                "protos/packet.proto",
                "protos/relayer.proto",
                "protos/searcher.proto",
                "protos/shared.proto",
            ],
            &["protos"],
        )
        .unwrap();
}
