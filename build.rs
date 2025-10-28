// fn main() {
//     tonic_prost_build::compile_protos("proto/hello.proto").unwrap();
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .file_descriptor_set_path("src/hello_descriptor.bin")
        .compile_protos(
            &["proto/hello.proto"],
            &["proto/hello"],
        )?;
    Ok(())
}