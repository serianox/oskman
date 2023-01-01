use std::process::Command;

fn build_ui() {
    Command::new("yarn")
        .arg("--cwd")
        .arg("ui")
        .arg("build")
        .spawn()
        .expect("yarn command failed to start");
}

fn main() {
    build_ui();

    tauri_build::build();
}
