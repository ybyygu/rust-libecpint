fn main() -> miette::Result<()> {
    // include paths
    let path = std::path::PathBuf::from("/usr/include/libecpint");
    let src = std::path::PathBuf::from("src");
    // example.cpp requires eigen source
    let eigen = std::path::PathBuf::from("/usr/include/eigen3");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path, &src, &eigen]).build()?;

    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++14").file("src/run.cpp").compile("run");

    // rebuild source if changed
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/run.cpp");
    println!("cargo:rerun-if-changed=src/run.h");

    // libecpint required link libraries
    println!("cargo:rustc-link-lib=ecpint");
    println!("cargo:rustc-link-lib=pugixml");
    println!("cargo:rustc-link-lib=Faddeeva");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
