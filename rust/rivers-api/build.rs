//! Build script that compiles `proto/rivers.proto` into Rust types via `tonic-prost-build`.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let mut config = tonic_prost_build::Config::new();
    config.protoc_executable(protoc);

    tonic_prost_build::configure().compile_with_config(
        config,
        &["../../proto/rivers.proto"],
        &["../../proto"],
    )?;

    Ok(())
}
