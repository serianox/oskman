include!("src/schemas.rs");

use std::env;
use std::fs;
use std::path::PathBuf;
// use std::process::Command;

fn generate_schema() {
    let schema = schemars::schema_for!(CustomResponse);
    let schema_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("../../../../../dist/schemas.json");
    schema_file.parent().map(|path| fs::create_dir_all(path));
    fs::write(schema_file.clone(), serde_json::to_string_pretty(&schema).unwrap()).unwrap();
  }

fn main() {
    generate_schema();
}
