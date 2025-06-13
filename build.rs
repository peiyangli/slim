fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let config =
        slint_build::CompilerConfiguration::new()
        .with_style("material".into()); //material, fluent, cupertino, cosmic, native
    slint_build::compile_with_config("ui/app_window.slint", config).expect("Slint build failed");
    // slint_build::compile("ui/app_window.slint").expect("Slint build failed");
    embed_resource::compile("ui/assets/app.rc", embed_resource::NONE).manifest_optional().unwrap();
}