fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    slint_build::compile("ui/app_window.slint").expect("Slint build failed");
    embed_resource::compile("ui/assets/app.rc", embed_resource::NONE).manifest_optional().unwrap();
}