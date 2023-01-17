// use pkg_config;
use std::env;
use std::path::PathBuf;

fn compile() {
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let _host = env::var("HOST").unwrap();
    let _target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-changed=miniprot/*.c");
    println!("cargo:rerun-if-changed=miniprot/*.h");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=z");

    let mut cc = cc::Build::new();
    cc.warnings(false);
    cc.out_dir(&out_path);
    cc.cpp_link_stdlib(None);
    cc.flag("-lpthread");
    cc.flag("-lz");
    cc.flag("-std=c99");
    // cc.static_flag(true);
    cc.opt_level(3);

    if let Some(include) = std::env::var_os("DEP_Z_INCLUDE") {
        println!("DEBUG: {:#?}", include);
        cc.include(include);
    }

    if let Ok(lib) = pkg_config::find_library("zlib") {
        for path in &lib.include_paths {
            cc.include(path);
        }
    }

    cc.include("miniprot");

    let files: Vec<_> = std::fs::read_dir("miniprot")
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect();

    assert!(files.len() != 0, "No files found in miniprot directory -- Did you forget to clone the submodule? git submodule init --recursive");

    for file in files {
        // Skip "main.c" and "example.c"
        if file.file_name().unwrap() == "main.c" || file.file_name().unwrap() == "example.c" {
            continue;
        }

        if let Some(x) = file.extension() {
            if x == "c" {
                cc.file(file);
            }
        }
    }

    cc.compile("libminiprot");
}

#[cfg(feature = "bindgen")]
fn gen_bindings() {
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header("miniprot.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Couldn't write bindings!")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to create bindings");
}

#[cfg(not(feature = "bindgen"))]
fn gen_bindings() {}

fn main() {
    compile();
    gen_bindings();
}
