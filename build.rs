include!("src/schema.rs");

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn generate_schema() {
    let schema = schemars::schema_for!(CustomResponse);
    let schema_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("schema.json");
    fs::write(schema_file.clone(), serde_json::to_string_pretty(&schema).unwrap()).unwrap();

    // Command::new("yarn")
    // .arg("--cwd")
    // .arg("ui")
    // .arg("run")
    // .arg("json2ts")
    // .arg("--")
    // .arg("--input")
    // .arg(schema_file.clone())
    // .arg("--output")
    // .arg("src/schema.d.ts")
    // .spawn()
    // .expect("yarn command failed to start");
  }

fn build_ui() {
  Command::new("yarn")
  .arg("--cwd")
  .arg("ui")
  .arg("build")
  .spawn()
  .expect("yarn command failed to start");
}

fn main() {
    generate_schema();

    build_ui();

    tauri_build::build();
}
