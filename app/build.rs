fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &["./protos/auth.proto", "./protos/chat.proto"], // paths to protos
            &["./protos"],                                    // include directory
        )?;
    Ok(())
}
