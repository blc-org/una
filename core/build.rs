use std::path::Path;

fn main() {
    let cargo_workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    println!("cargo:rustc-env=PROTOBUFS_DIR={}", Path::new(&cargo_workspace_dir).join("proto/out").display());
}
