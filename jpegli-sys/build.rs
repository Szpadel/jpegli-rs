use std::{
    env,
    path::{Path, PathBuf},
};

fn source_dir() -> PathBuf {
    env::var("DEP_JXL_PATH").map_or_else(
        |_| Path::new(env!("CARGO_MANIFEST_DIR")).join("libjxl"),
        PathBuf::from,
    )
}

pub fn main() {
    let source = source_dir();

    if let Ok(p) = std::thread::available_parallelism() {
        env::set_var("CMAKE_BUILD_PARALLEL_LEVEL", format!("{}", p))
    }

    let mut config = cmake::Config::new(source);
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("JPEGXL_ENABLE_JPEGLI", "ON")
        .define("JPEGLI_LIBJPEG_LIBRARY_SOVERSION", "8")
        .define("JPEGLI_LIBJPEG_LIBRARY_VERSION", "8.2.2")
        .build_target("jpegli-static");

    let mut prefix = config.build();
    prefix.push("build");
    prefix.push("lib");
    println!("cargo:rustc-link-search=native={}", prefix.display());

    let mut prefix = config.build();
    prefix.push("build");
    prefix.push("third_party");
    prefix.push("highway");
    println!("cargo:rustc-link-search=native={}", prefix.display());

    println!("cargo:rustc-link-lib=jpegli-static");
    println!("cargo:rustc-link-lib=hwy");

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_env = "msvc"
    )))]
    println!("cargo:rustc-link-lib=stdc++");
}
