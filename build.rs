use cmake::Config;
use std::path::Path;

fn main() {
    let project_name = "libs"; // libs remote or local ../name git --clone
    let native_dir = Path::new(project_name);

    // valid CMakeLists.txt file
    if !native_dir.join("CMakeLists.txt").exists() {
        panic!(
            "dont found 'CMakeLists.txt' in dir 'native/'. \
            get it: git clone <URL> native"
        );
    }

    let dst = Config::new(&native_dir).build();
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-search={}/lib64", dst.display());

    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=crpdfcpp");

    pkg_config::Config::new()
        .probe("cairo")
        .expect("Error: Dont found Cairo in system. install package libcairo2-dev or cairo-devel.");

    println!("cargo:rerun-if-changed={}/", project_name);
}