fn main() {
    let single_source = "src/par_shapes.h";

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let clang = cc::Build::new().try_get_compiler().unwrap();
    let sysroot = clang
        .path()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("sysroot")
        .to_string_lossy()
        .to_string();

    let mut compiler = cc::Build::new();
    compiler
        .flag("-std=c99")
        .flag("-xc")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-const-variable")
        .flag("-Wno-unneeded-internal-declaration")
        .define("PAR_SHAPES_IMPLEMENTATION", "1")
        .file(single_source);

    if target_os == "android" {
        compiler
            .flag(&format!("--sysroot={}", sysroot))
            .flag("-Wno-implicit-const-int-float-conversion");
    }

    compiler.compile("par_shapes");

    let builder = bindgen::builder()
        .clang_arg("-std=c99")
        .header(single_source)
        .allowlist_file(single_source)
        .layout_tests(false);

    let builder = if target_os == "android" {
        builder.clang_arg(format!("--sysroot={}", sysroot))
    } else {
        builder
    };

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
