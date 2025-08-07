use std::{fs, path::PathBuf};

fn main() {
    // // 链接库文件
    println!("cargo:rustc-link-search=native=../kmbox_libs/lib");
    println!("cargo:rustc-link-lib=dylib=kmboxAI");
    println!("cargo:rustc-link-lib=dylib=rknnrt");
    println!("cargo:rustc-link-lib=dylib=rga");

    // read ../kmbox_libs/include/kmboxAIlib/*
    let files = fs::read_dir("../kmbox_libs/include/kmboxAIlib").unwrap();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let bindings = bindgen::Builder::default()
            .clang_args(&[
                "-I/usr/include/opencv4/opencv",
                "-I/usr/include/opencv4",
                "-I/usr/include/c++/10",
                "-I/usr/include/x86_64-linux-gnu/c++/10",
                "-x",
                "c++",
                "--std=c++11",
            ])
            // The input header we would like to generate
            // bindings for.
            .header(format!("../kmbox_libs/include/kmboxAIlib/{}", file_name))
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        fs::create_dir_all("src/bindings").unwrap();
        let out_path = PathBuf::from(format!("src/bindings/{}.rs", file_name));
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
