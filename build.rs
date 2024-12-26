use std::env;
use std::path::Path;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&project_dir).join("lib");

    // Search path
    println!("cargo:rustc-link-search={}", lib_dir.display());

    // Link sherpa
    println!("cargo:rustc-link-lib=static=sherpa-onnx-c-api");
    println!("cargo:rustc-link-lib=static=sherpa-onnx-core");
    // println!("cargo:rustc-link-lib=static=kaldi-decoder-core");
    // println!("cargo:rustc-link-lib=static=kaldi-native-fbank-core");
    // println!("cargo:rustc-link-lib=static=sherpa-onnx-fst");
    // println!("cargo:rustc-link-lib=static=sherpa-onnx-fstfar");
    // println!("cargo:rustc-link-lib=static=sherpa-onnx-kaldifst-core");
    // println!("cargo:rustc-link-lib=static=ssentencepiece_core");

    // Link onnxruntime
    // println!("cargo:rustc-link-lib=static=onnxruntime");

    println!("cargo:rerun-if-changed=build.rs");
}
