extern crate cbindgen;

use cbindgen::RenameRule::CamelCase;
use cbindgen::{StructConfig, ParseConfig, SortKey::Name};

use std::env;
use std::path::PathBuf;
use cbindgen::Config;
use std::vec;


fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let structure = StructConfig{
        rename_fields : CamelCase,
        ..Default::default()
    };

    let parse = ParseConfig {
        parse_deps: true,
        include: Some(vec![String::from("reqwest::blocking")]),
        ..Default::default()
    };
    let config = Config {
        namespace: Some(String::from("ffi")),
        includes: vec![String::from("ffi.hpp")],
        pragma_once: true,
        cpp_compat:true,
        sort_by: Name,
        structure,
        parse,
        ..Default::default()
    };

    let _ = match cbindgen::generate_with_config(&crate_dir, config) {
        Ok(x) => x.write_to_file(&output_file),
        Err(e) => {println!("{:#?}", e); false}
    };
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}