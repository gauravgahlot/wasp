# wasp

Wasm Pipelines - Demo project for GopherCon

### Prerequisites

- [wabt](https://github.com/WebAssembly/wabt)
- [TinyGo](https://tinygo.org/)

### Usage

- Make help:

```sh
➜ make
Available targets:
  make help       Show this help
  make modules    Build all wasm modules
  make run        Build and run the pipeline
  make clean      Remove all generated wasm modules
```

- Build wasm modules for all tasks:

```sh
➜ make modules
wat2wasm tasks/init.wat -o tasks/modules/init.wasm
tinygo build -o tasks/modules/add.wasm -target=wasm-unknown -no-debug -opt=z tasks/add.go
rustc --target wasm32-unknown-unknown -O --crate-type=cdylib tasks/cube.rs -o tasks/modules/cube.wasm
rustc --target wasm32-unknown-unknown -O --crate-type=cdylib tasks/result.rs -o tasks/modules/result.wasm
```

- Execute the pipeline:

```
➜ make run
🚀 Running pipeline...
[info] Pipeline started: 'A polyglot WASM Pipeline'
  [info] Executing 'initialize' (WAT)
  [info] Executing 'add numbers' (Go)
  [info] Executing 'generate cube' (Rust)
  [info] Executing 'print result' (Rust)
  [info] Final result: 125
[info] Pipeline finished!
```
