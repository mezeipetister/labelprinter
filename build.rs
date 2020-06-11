use std::process::Command;

fn main() {
    // Compile Gresource
    let out = Command::new("glib-compile-resources")
        .args(&["--generate", "resources.xml"])
        .current_dir("data")
        .status()
        .expect("failed to generate resources");
    assert!(out.success());
}
