use std::process::{Command, Stdio};
use std::{env, path::Path};
use std::fs::{self, File};
// use std::io::Write;
// use std::path::PathBuf;

use regex::Regex;
use svd2rust::{Config, Target};
fn main() {
    // if env::var_os("CARGO_FEATURE_RT").is_some() {
        //     let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        //     File::create(out.join("device.x"))
        //         .unwrap()
    //         .write_all(include_bytes!("device.x"))
    //         .unwrap();
    //     println!("cargo:rustc-link-search={}", out.display());
    //     println!("cargo:rerun-if-changed=device.x");
    // }
    // Convert SVD file to Rust code
    const SVD_CONTENT: &str = include_str!("bl808.svd");
    let svd_config = Config {
        target: Target::RISCV,
        const_generic: true,
        derive_more: true,
        strict: false,
        make_mod: true,
        ..Default::default()
    };
    let generated = svd2rust::generate(SVD_CONTENT, &svd_config).unwrap();
    // Define destination for the generated code
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_file = Path::new(&out_dir).join("generated.rs");
    let dest_doc_file = Path::new(&out_dir).join("doc.txt");
    // Write content into destination
    fs::write(&dest_file, &generated.lib_rs).unwrap();
    // Format file
    let formatted = Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .stdin(File::open(&dest_file).unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap()
        .stdout;
    // Process generated code
    let mut lib_content = String::from_utf8(formatted).unwrap();
    let mut doc_content = String::new();
    if lib_content.starts_with("#![doc") {
        let mut lines = lib_content.lines();

        let doc_regex = Regex::new(r#"^#!\[doc = (".*")]$"#).unwrap();
        doc_content = lines.next().unwrap().to_owned();
        doc_content = doc_regex
            .captures(&doc_content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned();

        lib_content = lines.collect::<Vec<&str>>().join("\n");
    }
    // Write final generated data
    fs::write(&dest_file, lib_content).unwrap();
    // Write generated docs
    fs::write(&dest_doc_file, doc_content).unwrap();
}
