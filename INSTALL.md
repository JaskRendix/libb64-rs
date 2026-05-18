# **Installation & Build Guide**

This repository is a **Cargo workspace** containing:

- `b64/` — the Base64 library  
- `base64-cli/` — the command‑line tool  
- `examples/` — example binaries (`encode_file`, `decode_file`)

The workspace builds on any system with a stable Rust toolchain.

---

## **Requirements**

You need:

- Rust (stable)  
- Cargo (bundled with Rust)

Install Rust using rustup:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```
rustc --version
cargo --version
```

---

## **Building the Workspace**

Build all crates:

```
cargo build --workspace
```

Release build:

```
cargo build --release --workspace
```

---

## **Using the Library**

> ⚠️ **This crate is not yet published on crates.io.**  
> To use it, depend on it via Git or build the workspace locally.

Add it to another project via Git:

```toml
[dependencies]
b64 = { git = "https://github.com/your/repo.git" }
```

See the library usage examples for in‑memory and streaming APIs.

---

## **Building the Library (`b64`)**

Build only the library crate:

```
cargo build -p b64
```

Run its tests:

```
cargo test -p b64
```

Run its benchmarks:

```
cargo bench -p b64
```

Benchmark results are documented in **BENCHMARKS.md**.

---

## **Building the CLI Tool (`base64-cli`)**

Build the CLI tool:

```
cargo build --release -p base64-cli
```

This produces:

```
target/release/base64-cli
```

Usage:

```
base64-cli -e input.bin output.b64
base64-cli -d input.b64 output.bin
```

---

## **Running the Example Programs**

Example binaries live in `examples/src/bin/`.

Run them with Cargo:

```
cargo run --example encode_file -- input.bin out.b64
cargo run --example decode_file -- input.b64 out.bin
```

Build all examples:

```
cargo build --examples
```

---

## **Testing**

Run the full test suite across all crates:

```
cargo test --workspace
```

---

## **Notes**

- The library is portable and does not depend on platform‑specific features.  
- Parallel routines are included for experimentation.  
- The CLI tool is optional and built separately.  
- Example binaries demonstrate file‑based encoding and decoding.
