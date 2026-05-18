# **b64 — Base64 Encoding/Decoding in Rust**

> ⚠️ **Status:** This crate is **not yet published on crates.io**.  
> To use it today, depend on it via Git or build the workspace locally.

A clear, portable, and well‑tested Base64 implementation written in safe Rust.  
Includes in‑memory routines, streaming interfaces, optional parallel helpers, a CLI tool, and example binaries.

---

## **Features**
- In‑memory encode/decode (encode/decode)
- Streaming encode/decode (streaming API)
- Optional parallel routines
- CLI tool (`base64-cli`)
- Example binaries
- Benchmarks (see `BENCHMARKS.md`)

---

## **Workspace Layout**
This repository is a Cargo workspace containing:

- `b64/` — the library  
- `base64-cli/` — the command‑line tool  
- `examples/` — example binaries

Build everything:

```
cargo build --workspace
```

---

## **Using the Library**

Since the crate is **not yet on crates.io**, use a Git dependency:

```toml
[dependencies]
b64 = { git = "https://github.com/your/repo.git" }
```

Or build locally.

### **In‑memory API**

```rust
use b64::{encode_to_string, decode_to_vec};

let encoded = encode_to_string(b"hello");
let decoded = decode_to_vec(&encoded).unwrap();
```

### **Streaming API**

```rust
use b64::encode_reader_to_writer;
use std::fs::File;

let mut input = File::open("in.bin")?;
let mut output = File::create("out.b64")?;
encode_reader_to_writer(&mut input, &mut output, None)?;
```

---

## **CLI Tool**

Build the CLI tool:

```
cargo build --release -p base64-cli
```

Encode:

```
base64-cli -e input.bin output.b64
```

Decode:

```
base64-cli -d input.b64 output.bin
```

---

## **Examples**

Run example binaries:

```
cargo run --example encode_file -- input.bin out.b64
cargo run --example decode_file -- input.b64 out.bin
```

---

## **Tests & Benchmarks**

Run tests:

```
cargo test --workspace
```

Run benchmarks:

```
cargo bench -p b64
```

Benchmark results are in `BENCHMARKS.md`.

---

## **Platform Support**

Tested on:

- Linux (Ubuntu 22.04)
- Windows 10 (MSVC)
- macOS Ventura

Portable and requires only stable Rust.

---

## **License**

Released into the **Public Domain**.  
See `LICENSE.md`.
