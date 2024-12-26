# sherpa-wasm-repro

## Prepare repository

```console
git clone https://github.com/thewh1teagle/sherpa-wasm-repro
cd sherpa-wasm-repro
mkdir lib
```

## Install emcc

```console
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
git pull
./emsdk install 3.1.53
./emsdk activate 3.1.53
source ./emsdk_env.sh
export EMSCRIPTEN=$(dirname $(realpath $(which emcc)))
export SHERPA_ONNX_IS_USING_BUILD_WASM_SH=ON
echo $EMSCRIPTEN
```

## Build sherpa-onnx

```console
git clone https://github.com/k2-fsa/sherpa-onnx
cd sherpa-onnx
mkdir build
cmake \
  -B build -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_TOOLCHAIN_FILE=$EMSCRIPTEN/cmake/Modules/Platform/Emscripten.cmake \
  -DSHERPA_ONNX_ENABLE_C_API=ON \
  -DSHERPA_ONNX_ENABLE_WASM=ON \
  -DSHERPA_ONNX_ENABLE_PYTHON=OFF \
  -DSHERPA_ONNX_ENABLE_TESTS=OFF \
  -DSHERPA_ONNX_ENABLE_CHECK=OFF \
  -DBUILD_SHARED_LIBS=OFF \
  -DSHERPA_ONNX_ENABLE_PORTAUDIO=OFF \
  -DSHERPA_ONNX_ENABLE_JNI=OFF \
  -DSHERPA_ONNX_ENABLE_TTS=OFF \
  -DSHERPA_ONNX_ENABLE_WEBSOCKET=OFF \
  -DSHERPA_ONNX_ENABLE_GPU=OFF \
  -DSHERPA_ONNX_ENABLE_WASM_ASR=OFF \
  -DSHERPA_ONNX_ENABLE_BINARY=OFF \
  -DSHERPA_ONNX_LINK_LIBSTDCPP_STATICALLY=OFF
cmake --build build
cp build/_deps/onnxruntime-src/lib/libonnxruntime.a build/lib/
cp -rf build/lib ../lib
```

## Build

Install [wasm-pack](https://rustwasm.github.io/wasm-pack)

```console
RUSTFLAGS="--cfg=web_sys_unstable_apis --Z wasm_c_abi=spec" rustup run nightly wasm-pack build --target web
```

## Related issues

- https://github.com/rustwasm/wasm-pack/issues/621#issuecomment-481703973
- https://github.com/rustwasm/team/issues/291#issuecomment-2138201722
- https://github.com/rustwasm/wasm-bindgen/issues/3315
- https://github.com/pykeio/ort/issues/75
- https://github.com/rustwasm/wasm-bindgen/issues/2215#issuecomment-648205478
- https://stackoverflow.com/questions/78556000/how-you-can-link-a-rust-library-with-c-c-and-wasm-bindgen-for-the-wasm32-unkno

## Run index.html

```console
python3 -m http.server 80
```

Open http://localhost