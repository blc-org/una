use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    std::env::set_var("OUT_DIR", Path::new(&cargo_workspace_dir).join("proto/out"));

    tonic_build::configure()
        .build_server(false)
        .compile(
            &[Path::new(&cargo_workspace_dir).join("proto/cln-v0.11.2/node.proto")],
            &[Path::new(&cargo_workspace_dir).join("proto/cln-v0.11.2")],
        )?;

    Ok(())
}
