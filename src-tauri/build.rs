fn windows() -> tauri_build::WindowsAttributes {
  let manifest = std::fs::read_to_string("./window-app-manifest.xml").unwrap();
  tauri_build::WindowsAttributes::new().app_manifest(manifest)
}

fn main() {
  let attrs = tauri_build::Attributes::new().windows_attributes(windows());
  tauri_build::try_build(attrs)
    .expect("Failed to run build script");
}
