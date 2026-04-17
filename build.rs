fn main() {
    // Embed CJK font if present; set cfg flag for conditional compilation in main.rs
    if std::path::Path::new("assets/fonts/NotoSansSC-Regular.ttf").exists() {
        println!("cargo:rustc-cfg=has_cjk_font");
    }
    println!("cargo:rerun-if-changed=assets/fonts/NotoSansSC-Regular.ttf");
}
