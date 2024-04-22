use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let uhal_lib_dir = PathBuf::from("./ipbus-software/uhal/uhal/")
        .canonicalize()
        .expect("Could not get path to uhal");

    let header_dir = uhal_lib_dir.join("include");
    let header_path = header_dir.join("uhal").join("uhal.hpp");
    let lib_path = uhal_lib_dir.join("lib");

    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=cactus_uhal_uhal");

    if !Command::new("make")
        .current_dir(uhal_lib_dir.to_str().unwrap())
        .arg("all")
        .arg(format!("-j{}", num_cpus::get()))
        .output()
        .expect("Could not spawn 'make' to build uhal.")
        .status
        .success()
    {
        panic!("Could not build 'uhal'");
    }

    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{}", header_dir.to_str().unwrap()))
        .clang_arg(format!(
            "-I{}",
            uhal_lib_dir.join("../log/include").to_str().unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            uhal_lib_dir.join("../grammars/include").to_str().unwrap()
        ))
        .generate()
        .expect("Unable to generate bindings to uhal");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}
