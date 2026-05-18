# Base64 Benchmark Report  
### libb64‑rust performance evaluation  
### Machine: Acer Aspire V3‑572G (Intel i7‑4510U), Ubuntu Linux

---

## Input  
Binary file: **18 MB** (`bigfile.bin`)

---

## Method  
Each benchmark measures throughput in MiB/s.  
Criterion handles warm‑up and sampling.  
All results come from `cargo bench`.

---

# Results

## 1. In‑memory encoding

| Operation | Time | Throughput |
|----------|------|------------|
| encode_to_string | 104–110 ms | **158–164 MiB/s** |

---

## 2. In‑memory decoding

| Operation | Time | Throughput |
|----------|------|------------|
| decode_to_vec | 118–119 ms | **144–145 MiB/s** |

---

## 3. File‑based encoding

### Unbuffered

| Time | Throughput |
|------|------------|
| 377–390 ms | **43–45 MiB/s** |

### Buffered (128 KiB)

| Time | Throughput |
|------|------------|
| 365–377 ms | **45–47 MiB/s** |

---

## 4. File‑based decoding

### Unbuffered

| Time | Throughput |
|------|------------|
| 332–342 ms | **50–51 MiB/s** |

### Buffered (128 KiB)

| Time | Throughput |
|------|------------|
| 294–307 ms | **55–58 MiB/s** |

---

## 5. Memory‑mapped I/O

### Encode (mmap)

| Time | Throughput |
|------|------------|
| 110–112 ms | **152–154 MiB/s** |

### Decode (mmap)

| Time | Throughput |
|------|------------|
| 118–118.6 ms | **144–145 MiB/s** |

---

## 6. Parallel implementation

### Encode (parallel)

| Time | Throughput |
|------|------------|
| 11.13–11.36 ms | **1.47–1.50 GiB/s** |

### Decode (parallel)

| Time | Throughput |
|------|------------|
| 74.19–74.55 ms | **230–231 MiB/s** |

Parallel encode is fast due to AVX2 and Rayon.  
Parallel decode is slower because decode has more dependency chains.

---

## 7. Encode + Decode Roundtrip (in‑memory)

| Operation | Time | Throughput |
|----------|-------|------------|
| encode_then_decode | 225–226 ms | **~76 MiB/s** |

---

## 8. Parallel Roundtrip

| Operation | Time | Throughput |
|----------|-------|------------|
| parallel_encode_then_decode | 107–110 ms | **~155–160 MiB/s** |

Parallel decode dominates the cost.

---

## 9. Scalar vs Parallel Comparison

| Operation | Time | Throughput |
|----------|-------|------------|
| scalar_encode | 106–110 ms | **155–160 MiB/s** |
| parallel_encode | 11.11–11.26 ms | **1.49–1.50 GiB/s** |

Parallel encode is roughly **9× faster**.

---

## 10. Encode + Decode Loop (50×)

This benchmark matches the original libb64 method.

| Operation | Time |
|----------|-------|
| encode+decode 50× | **11.31–11.36 s** |

### Throughput calculation

```
900 MB / 11.33 s ≈ 79.4 MB/s
```

---

# Summary

| Operation | Throughput |
|-----------|------------|
| Encode (in‑memory) | **~160 MiB/s** |
| Decode (in‑memory) | **~145 MiB/s** |
| Encode (file) | 45–47 MiB/s |
| Decode (file) | 55–58 MiB/s |
| Encode (mmap) | 152–154 MiB/s |
| Decode (mmap) | 144–145 MiB/s |
| Encode (parallel) | **1.49 GiB/s** |
| Decode (parallel) | 230–231 MiB/s |
| Roundtrip (scalar) | **~76 MiB/s** |
| Roundtrip (parallel) | **~158 MiB/s** |
| Encode+Decode loop (50×) | **~79 MB/s** |

---

# Conclusion

- The scalar Rust implementation reaches **~160 MiB/s** for encoding and **~145 MiB/s** for decoding.  
- File I/O limits throughput to **45–58 MiB/s**.  
- Memory‑mapped I/O matches in‑memory performance.  
- Parallel encode reaches **~1.5 GiB/s** using AVX2 and Rayon.  
- Parallel decode reaches **~230 MiB/s**.  
- Combined encode+decode throughput is **~76–80 MB/s**.  

---

## Comparison with original C libb64

### Original C results (Pentium M @ 2 GHz)

| Implementation | Total Time (50×) | Throughput |
|----------------|------------------|------------|
| libb64‑1.2     | 28.389 s         | 31.7 MB/s  |
| coreutils      | 36.288 s         | 24.8 MB/s  |
| fourmilab      | 103.160 s        | 8.7 MB/s   |

### Rust libb64 results (i7‑4510U)

| Operation | Time | Throughput |
|-----------|------|------------|
| encode+decode 50× | 11.31–11.36 s | ~79 MB/s |

### Summary

- Rust libb64: **~79 MB/s**  
- C libb64‑1.2: **31.7 MB/s**  
- coreutils base64: **24.8 MB/s**  
- fourmilab base64: **8.7 MB/s**  

The Rust implementation is faster than the original C version on a per‑core basis.
