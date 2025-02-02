#!/usr/bin/env -S nu --stdin

def main [
  target: string # Target "wasm"
  ] {
    if ($target | is-empty) {
      help main | print -e
      exit 1
    }

  if ($target == 'wasm') {
    let args = [
      "-O3", 
      "-sUSE_GLFW=3", 
      "-sGL_ENABLE_GET_PROC_ADDRESS", 
      "-sWASM=1", 
      "-sALLOW_MEMORY_GROWTH=1", 
      "-sWASM_MEM_MAX=512MB", 
      "-sTOTAL_MEMORY=512MB", 
      "-sABORTING_MALLOC=0", 
      "-sASYNCIFY", 
      "-sFORCE_FILESYSTEM=1", 
      "-sASSERTIONS=1", 
      "-sERROR_ON_UNDEFINED_SYMBOLS=0", 
      "-sEXPORTED_RUNTIME_METHODS=ccallcwrap", 
      "--preload-file",
      "assets@assets",
    ] | str join " "

    # EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap --preloa-file assets/background@assets" cargo build --release --target wasm32-unknown-emscripten
    print $args
    EMCC_CFLAGS=$args cargo build --release --target wasm32-unknown-emscripten

    print "Copy builded to dist..."
    rm -rf ./dist/*
    mkdir ./dist
    cp ./web/* ./dist
    cp ./target/wasm32-unknown-emscripten/release/deps/*.data ./dist
    cp ./target/wasm32-unknown-emscripten/release/*.wasm ./dist
    cp ./target/wasm32-unknown-emscripten/release/*.js ./dist
    exit 0
  }
}

