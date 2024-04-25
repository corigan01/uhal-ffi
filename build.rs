// use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> miette::Result<()> {
    let uhal_lib_dir = PathBuf::from("./ipbus-software/uhal/uhal/")
        .canonicalize()
        .expect("Could not get path to uhal");

    println!(
        "cargo::rustc-link-search={}",
        uhal_lib_dir.join("lib").to_str().unwrap()
    );
    println!(
        "cargo::rustc-link-search={}",
        uhal_lib_dir.join("../log/lib").to_str().unwrap()
    );
    println!(
        "cargo::rustc-link-search={}",
        uhal_lib_dir.join("../grammars/lib").to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=static=cactus_uhal_uhal");
    println!("cargo:rustc-link-lib=static=cactus_uhal_grammars");
    println!("cargo:rustc-link-lib=static=cactus_uhal_log");

    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=pugixml");

    println!("cargo:rustc-link-lib=boost_filesystem");
    println!("cargo:rustc-link-lib=boost_regex");
    println!("cargo:rustc-link-lib=boost_system");
    println!("cargo:rustc-link-lib=boost_chrono");

    println!("cargo:rerun-if-changed=src/ffi.rs");

    Command::new("make")
        .current_dir(uhal_lib_dir.join("../").to_str().unwrap())
        .env("BUILD_STATIC", "1")
        .arg("all")
        .arg(format!("-j{}", num_cpus::get()))
        .output()
        .expect("Unable to invoke 'make'");

    autocxx_build::Builder::new(
        "src/ffi.rs",
        &[
            "extra-cpp",
            "ipbus-software/uhal/log/include",
            "ipbus-software/uhal/grammars/include",
            "ipbus-software/uhal/uhal/include",
        ],
    )
    .build()?
    .compile("uhalbind");

    Ok(())
}
