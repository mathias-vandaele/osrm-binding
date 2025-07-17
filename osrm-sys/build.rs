use std::path::PathBuf;

fn main() {
    let osrm_source_path = PathBuf::from("third_party/osrm-backend");
    let cxx_flags = "-Wno-array-bounds -Wno-uninitialized -Wno-stringop-overflow -std=c++17";

    let dst = cmake::Config::new("third_party/osrm-backend")
        .env("CXXFLAGS", cxx_flags) 
        .build();

    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .flag("-std=c++17")
        .include(dst.join("include"))
        .include(osrm_source_path.join("include"))
        .include(osrm_source_path.join("third_party/fmt/include"))
        .compile("osrm_wrapper");

    let lib_path = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    println!("cargo:rustc-link-lib=static=osrm_store");
    println!("cargo:rustc-link-lib=static=osrm_extract");
    println!("cargo:rustc-link-lib=static=osrm_partition");
    println!("cargo:rustc-link-lib=static=osrm_update");
    println!("cargo:rustc-link-lib=static=osrm_guidance");
    println!("cargo:rustc-link-lib=static=osrm_customize");
    println!("cargo:rustc-link-lib=static=osrm_contract");
    println!("cargo:rustc-link-lib=static=osrm");

    println!("cargo:rustc-link-lib=dylib=boost_thread");
    println!("cargo:rustc-link-lib=dylib=boost_filesystem");
    println!("cargo:rustc-link-lib=dylib=boost_iostreams");
    println!("cargo:rustc-link-lib=dylib=tbb");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=bz2");
    println!("cargo:rustc-link-lib=dylib=expat");
}