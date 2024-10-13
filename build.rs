use std::path::PathBuf;

fn main(){
    println!("cargo:rerun-if-changed=ch347_lib.h");
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=dylib=ch347");
    let bindings = bindgen::Builder::default()
        .header("lib/ch347_lib.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("lib");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}