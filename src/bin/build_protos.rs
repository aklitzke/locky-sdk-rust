fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/autogenerated")
        .compile(&["proto/api.proto"], &["proto/"])
        .unwrap();
}
