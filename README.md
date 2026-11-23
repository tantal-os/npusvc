# npusvc

Rust runtime for selecting a CPU, GPU, or NPU and running TFLite models through
the npusvc SDK. It loads LiteRT from a caller-supplied directory and uses the
plugin and cache directories supplied by the caller.

Create a `Runtime` to load LiteRT, compile a model with a `Device`, then set
the model inputs, call `run`, and read its outputs. The Tantal NPU research CLI
uses this library for its inference experiments.

```sh
cargo build --locked
cargo test --locked
```

For Android arm64, install the `aarch64-linux-android` Rust target and set
`CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` to the NDK's
`aarch64-linux-android36-clang` before building.

```sh
cargo build --locked --release --target aarch64-linux-android
```
