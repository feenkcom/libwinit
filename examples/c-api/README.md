# Sample C program using Winit bindings

## Generating `winit.h` header

Make sure to install latest [cbindgen](https://github.com/eqrion/cbindgen):
```bash
cargo install cbindgen
```

Generate the header running the following from the root of `libwinit` repository:
```bash
cbindgen --config cbindgen.toml --crate libwinit --output winit.h
```

Compile `libWinit`:
```bash
cargo build --package libwinit --release
```

Copy compiled shared library and header in the same folder as `main.c`
```bash
# MacOS:
cp target/release/libWinit.dylib examples/c-api/libWinit.dylib
cp winit.h examples/c-api/winit.h
# Linux:
cp target/release/libWinit.so examples/c-api/libWinit.so
cp winit.h examples/c-api/winit.h
```

Compile `main.c` linking `libWinit`:
```bash
gcc main.c -lWinit -L . -o main
```

Run the example:
```bash
./main
```