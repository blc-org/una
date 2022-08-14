fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/cln-v0.11.2/node.proto")?;
    Ok(())
}
