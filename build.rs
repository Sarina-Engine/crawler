fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/scraper_rpc.proto")?;
    tonic_build::compile_protos("proto/product_send.proto")?;
    tonic_build::compile_protos("proto/category_send.proto")?;
    tonic_build::compile_protos("proto/comment_send.proto")?;
    Ok(())
}
