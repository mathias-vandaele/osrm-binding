use std::io::Cursor;
use std::path::{Path, PathBuf};


fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let osrm_url = "https://github.com/Project-OSRM/osrm-backend/archive/refs/tags/v6.0.0.tar.gz";

    eprintln!("Downloading OSRM source from {}...", osrm_url);

    let mut response = reqwest::blocking::get(osrm_url).unwrap();
    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();

    eprintln!("Decompressing OSRM source...");
    let cursor = Cursor::new(buffer);
    let tar_gz = flate2::read::GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(tar_gz);
    archive.unpack(&out_dir).unwrap();

    let osrm_source_path = find_osrm_source(&out_dir);
    eprintln!("OSRM source path: {}", osrm_source_path.display());

    let cxx_flags = "-Wno-array-bounds -Wno-uninitialized -Wno-stringop-overflow -std=c++17";

    let dst = cmake::Config::new(&osrm_source_path)
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

fn find_osrm_source(path: &Path) -> PathBuf {
    for entry in path.read_dir().expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("osrm-backend-") {
            return path;
        }
    }
    panic!("Could not find OSRM source directory");
}