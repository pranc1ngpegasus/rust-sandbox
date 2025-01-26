use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("file_descriptor.bin"))
        .compile_protos(&["user/v1/service.proto"], &["."])?;

    Ok(())
}
