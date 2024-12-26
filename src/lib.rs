mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

/*
*** Sherpa related ***
 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SherpaOnnxWave {
    pub samples: *const f32,
    pub sample_rate: i32,
    pub num_samples: i32,
}

extern "C" {
    pub fn SherpaOnnxReadWave(filename: *const ::std::os::raw::c_char) -> *const SherpaOnnxWave;
}

#[wasm_bindgen]
pub fn read_wave() {
    let path = std::ffi::CString::new("test").unwrap();
    unsafe {
        SherpaOnnxReadWave(path.as_ptr());
    }
}
