use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mnn_inclued_dir = env::var("MNN_INCLUDE_DIR")
        .map(|dir| PathBuf::from(dir))
        .expect(
            "ERROR: please set NNN_INCLUDE_DIR,e.g. export MNN_INCLUDE_DIR=/path/to/mnn/include",
        );
    let bindings = bindgen::Builder::default()
        .header(format!("{}/MNN/Tensor.hpp", mnn_inclued_dir.display())) // #
        .header(format!("{}/MNN/Interpreter.hpp", mnn_inclued_dir.display())) //
        .clang_arg(format!("-I{}", mnn_inclued_dir.display())) // 添加这一行
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .allowlist_type("regex")
        .allowlist_function("MNN.*")
        .allowlist_var("MNN.*")
        .allowlist_type("MNN.*")
        // 忽略无用类型
        .blocklist_type("_Base")
        // 生成 Rust 代码中的构造函数
        // .opaque_type("std::.*")
        .opaque_type("std::vector")
        .generate_comments(false)
        .layout_tests(false) // 关闭结构体测试#[test]
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
