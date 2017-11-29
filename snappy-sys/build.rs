extern crate pkg_config;
extern crate cmake;

use std::env;
use std::fs;

use cmake::Config;

fn main() {
    let src = env::current_dir().unwrap().join("snappy");
    let dst = Config::new("snappy").build_target("snappy").build();
    let build = dst.join("build");
    println!("cargo:root={}", build.display());
    println!("cargo:rustc-link-lib=static=snappy");
    println!("cargo:rustc-link-search=native={}", build.display());
    fs::copy(src.join("snappy.h"), build.join("snappy.h")).unwrap();
    configure_stdcpp();
}

fn configure_stdcpp() {
    // From: https://github.com/alexcrichton/gcc-rs/blob/master/src/lib.rs
    let target = env::var("TARGET").unwrap();
    let cpp = if target.contains("darwin") { "c++" } else { "stdc++" };
    println!("cargo:rustc-link-lib={}", cpp);
}
