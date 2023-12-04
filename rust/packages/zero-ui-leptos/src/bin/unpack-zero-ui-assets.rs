pub fn main() {
    let root_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let _ = zero_ui::unpack_assets(root_dir.join("assets"));
}

