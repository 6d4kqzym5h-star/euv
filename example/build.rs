use std::{env, fs::read_to_string, path::PathBuf};

use {chrono::Local, toml::Table};

const ENV_KEY_EUV_PACKAGE_NAME_KEY: &str = "EUV_PACKAGE_NAME";
const ENV_KEY_EUV_VERSION_KEY: &str = "EUV_VERSION";
const ENV_KEY_EUV_BUILD_TIME_KEY: &str = "EUV_BUILD_TIME";

fn main() {
    let manifest_dir: String = env!("CARGO_MANIFEST_DIR").to_string();
    let toml_path: PathBuf = PathBuf::from(manifest_dir).join("Cargo.toml");
    let toml_content: String = read_to_string(&toml_path).expect("Failed to read Cargo.toml");
    let toml_table: Table = toml_content
        .parse::<Table>()
        .expect("Failed to parse Cargo.toml");
    let package: &Table = toml_table
        .get("package")
        .expect("Missing [package] section")
        .as_table()
        .expect("Package section is not a table");
    let package_name: &str = package
        .get("name")
        .expect("Missing name field")
        .as_str()
        .expect("Name is not a string");
    let version_value: &str = package
        .get("version")
        .expect("Missing version field")
        .as_str()
        .expect("Version is not a string");
    let build_time_formatted: String = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S%.6f"));
    println!("cargo:rustc-env={ENV_KEY_EUV_PACKAGE_NAME_KEY}={package_name}");
    println!("cargo:rustc-env={ENV_KEY_EUV_VERSION_KEY}={version_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_TIME_KEY}={build_time_formatted}");
}
