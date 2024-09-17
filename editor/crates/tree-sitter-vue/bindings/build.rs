fn main() {
    let src_dir = std::path::Path::new("src");

    let mut c_config = cc::Build::new();

    c_config
        .std("c11")
        .include(src_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    c_config.compile("parser");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let mut c_config = cc::Build::new();

    c_config
        .cpp(true)
        .include(&src_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");

    let scanner_path = src_dir.join("scanner.cc");
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    let binder_dir = src_dir.join("binder.cc");
    c_config.file(&binder_dir);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    c_config.compile("tree-sitter-html");
}
