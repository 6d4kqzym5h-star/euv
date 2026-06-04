use std::{
    env,
    fs::read_to_string,
    fs::write,
    path::{Display, PathBuf},
};

use {chrono::Local, toml::Table};

const ENV_KEY_EUV_PACKAGE_NAME_KEY: &str = "EUV_PACKAGE_NAME";
const ENV_KEY_EUV_VERSION_KEY: &str = "EUV_VERSION";
const ENV_KEY_EUV_DESCRIPTION_KEY: &str = "EUV_DESCRIPTION";
const ENV_KEY_EUV_REPOSITORY_KEY: &str = "EUV_REPOSITORY";
const ENV_KEY_EUV_AUTHORS_KEY: &str = "EUV_AUTHORS";
const ENV_KEY_EUV_LICENSE_KEY: &str = "EUV_LICENSE";
const ENV_KEY_EUV_EDITION_KEY: &str = "EUV_EDITION";
const ENV_KEY_EUV_REPOSITORY_NAME_KEY: &str = "EUV_REPOSITORY_NAME";
const ENV_KEY_EUV_BUILD_TIME_KEY: &str = "EUV_BUILD_TIME";
const ENV_KEY_EUV_BUILD_DATE_KEY: &str = "EUV_BUILD_DATE";
const ENV_KEY_EUV_BUILD_CLOCK_KEY: &str = "EUV_BUILD_CLOCK";
const ENV_KEY_EUV_BUILD_TIMESTAMP_KEY: &str = "EUV_BUILD_TIMESTAMP";
const BUILD_STATE_FILE_NAME: &str = ".euv_build_state";

fn main() {
    let manifest_dir: String = env!("CARGO_MANIFEST_DIR").to_string();
    let state_file_path: PathBuf = PathBuf::from(&manifest_dir).join(BUILD_STATE_FILE_NAME);
    let state_file_path_display: Display<'_> = state_file_path.display();
    println!("cargo:rerun-if-changed={state_file_path_display}");
    let toml_path: PathBuf = PathBuf::from(&manifest_dir).join("Cargo.toml");
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
    let description_value: &str = package
        .get("description")
        .expect("Missing description field")
        .as_str()
        .expect("Description is not a string");
    let repository_value: &str = package
        .get("repository")
        .expect("Missing repository field")
        .as_str()
        .expect("Repository is not a string");
    let authors_value: String = package
        .get("authors")
        .and_then(|value: &toml::Value| value.as_array())
        .map(|array: &Vec<toml::Value>| {
            array
                .iter()
                .filter_map(|item: &toml::Value| item.as_str().map(|s: &str| s.to_string()))
                .collect::<Vec<String>>()
                .join(", ")
        })
        .unwrap_or_default();
    let license_value: &str = package
        .get("license")
        .expect("Missing license field")
        .as_str()
        .expect("License is not a string");
    let edition_value: &str = package
        .get("edition")
        .expect("Missing edition field")
        .as_str()
        .expect("Edition is not a string");
    let repository_name_value: String = repository_value
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('/')
        .rev()
        .take(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("/");
    let build_time_formatted: String = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S%.6f"));
    let build_date: String = format!("{}", Local::now().format("%Y-%m-%d"));
    let build_clock: String = format!("{}", Local::now().format("%H:%M:%S"));
    let build_timestamp: String = format!("{}", Local::now().timestamp_micros());
    write(&state_file_path, &build_time_formatted).expect("Failed to write build state file");
    println!("cargo:rustc-env={ENV_KEY_EUV_PACKAGE_NAME_KEY}={package_name}");
    println!("cargo:rustc-env={ENV_KEY_EUV_VERSION_KEY}={version_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_DESCRIPTION_KEY}={description_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_REPOSITORY_KEY}={repository_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_AUTHORS_KEY}={authors_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_LICENSE_KEY}={license_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_EDITION_KEY}={edition_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_REPOSITORY_NAME_KEY}={repository_name_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_TIME_KEY}={build_time_formatted}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_DATE_KEY}={build_date}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_CLOCK_KEY}={build_clock}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_TIMESTAMP_KEY}={build_timestamp}");
}
