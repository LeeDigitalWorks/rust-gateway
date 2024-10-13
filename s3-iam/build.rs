use glob::glob;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let protos: Vec<PathBuf> = glob("proto/*.proto")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let mut config = prost_build::Config::new();
    config.disable_comments(["."]);

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_well_known_types(true)
        .out_dir("src/iampb")
        .compile_protos(&protos, &["proto"])?;

    Ok(())
}
