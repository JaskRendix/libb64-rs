# libb64‑rust: Base64 Encoding/Decoding Routines

## Requirements

This library has minimal requirements.

It builds on any system with a stable Rust toolchain.  
It has been tested on:

- Linux  
  - Ubuntu 22.04  
  - Rust stable  
  - cargo 1.x  
- Windows  
  - Windows 10  
  - MSVC toolchain  
- macOS  
  - macOS Ventura  
  - Apple Clang + Rust stable  

The code is portable and does not depend on platform‑specific features.

If you build it on an unusual architecture or OS, feel free to report it.

---

## Compiling

There is no configure script.

To build the library:

```
cargo build
```

To run the test suite:

```
cargo test
```

To run the benchmarks:

```
cargo bench
```

---

## Installing

This crate is intended to be used as a library.  
Add it to your project with:

```
cargo add b64
```

The repository also includes small example binaries for file‑based encoding and decoding.  
You can build them with:

```
cargo build --release --bin encode_file
cargo build --release --bin decode_file
```

---

## Notes

The goal of this project is to provide a clear, correct, and well‑tested Base64 implementation in Rust.  
The API includes:

- in‑memory encode/decode  
- streaming encode/decode  
- file‑based encode/decode  
- optional parallel routines  
- a small CLI tool  

If you run it on a platform not listed above, or if you find an edge case, patches and reports are welcome.
