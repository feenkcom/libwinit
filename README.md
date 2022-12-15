# libWinit
C style wrapper for [Winit](https://github.com/rust-windowing/winit/) - cross-platform window creation and management in Rust.

## Generating `winit.h` header

Make sure to install latest [cbindgen](https://github.com/eqrion/cbindgen):
```bash
cargo install cbindgen
```

Generate the header running the following from the root of `libwinit` repository:
```bash
cbindgen --config cbindgen.toml --crate libwinit --output winit.h
```

## Released `winit.h` header
`winit.h` is released together with shared libraries.
The latest header is available on GitHub: https://github.com/feenkcom/libwinit/releases/latest

## Example C project
Check [Example using C API](https://github.com/feenkcom/libwinit/blob/main/examples/c-api/README.md) to learn how to compile a sample C program to open a native window.