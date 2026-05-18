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
| encode_to_string | 110–117 ms | **146–156 MiB/s** |

---

## 2. In‑memory decoding

| Operation | Time | Throughput |
|----------|------|------------|
| decode_to_vec | 129–135 ms | **127–132 MiB/s** |

---

## 3. File‑based encoding

### Unbuffered

| Time | Throughput |
|------|------------|
| 359–377 ms | **45–48 MiB/s** |

### Buffered (128 KiB)

| Time | Throughput |
|------|------------|
| 351–368 ms | **46–49 MiB/s** |

---

## 4. File‑based decoding

### Unbuffered

| Time | Throughput |
|------|------------|
| 300–313 ms | **54–57 MiB/s** |

### Buffered (128 KiB)

| Time | Throughput |
|------|------------|
| 300–314 ms | **55–57 MiB/s** |

---

## 5. Memory‑mapped I/O

### Encode (mmap)

| Time | Throughput |
|------|------------|
| 115–119 ms | **144–149 MiB/s** |

### Decode (mmap)

| Time | Throughput |
|------|------------|
| 121–122 ms | **141 MiB/s** |

---

## 6. Parallel implementation

### Encode (parallel)

| Time | Throughput |
|------|------------|
| 401–407 ms | **42–43 MiB/s** |

### Decode (parallel)

| Time | Throughput |
|------|------------|
| 672–678 ms | **25–26 MiB/s** |

Parallel performance is slower than serial.  
Base64 is memory‑bound and does not scale across threads.

---

## 7. Encode + Decode Loop (50×)

This benchmark matches the original libb64 method.

| Operation | Time |
|----------|-------|
| encode+decode 50× | **11.55–12.07 s** |

### Throughput calculation

Total data processed per iteration:

```
50 × 18 MB encode
50 × 18 MB decode
= 900 MB
```

Throughput:

```
900 MB / 11.78 s ≈ 76.4 MB/s
```

This includes both encode and decode in one loop.

---

# Summary

| Operation | Throughput |
|-----------|------------|
| Encode (in‑memory) | **~150 MiB/s** |
| Decode (in‑memory) | **~130 MiB/s** |
| Encode (file) | 46–49 MiB/s |
| Decode (file) | 55–57 MiB/s |
| Encode (mmap) | 144–149 MiB/s |
| Decode (mmap) | 141 MiB/s |
| Encode (parallel) | 42–43 MiB/s |
| Decode (parallel) | 25–26 MiB/s |
| Encode+Decode loop (50×) | **~76 MB/s** |

---

# Conclusion

- The serial Rust implementation reaches **~150 MiB/s** for encoding and **~130 MiB/s** for decoding in memory.  
- File I/O limits throughput to **45–57 MiB/s**.  
- Memory‑mapped I/O matches in‑memory performance.  
- The parallel version is slower due to overhead and poor scaling for Base64 workloads.  
- The encode+decode loop processes **900 MB in ~12 s**, giving **~76 MB/s** combined throughput.  

---

## Comparison with original C libb64

The original libb64 benchmark (2010) encoded and decoded an 18 MB file 50 times.  
Total data processed per run: 900 MB.

### Original C results (Pentium M @ 2 GHz)

| Implementation | Total Time (50×) | Throughput |
|----------------|------------------|------------|
| libb64‑1.2     | 28.389 s         | 31.7 MB/s  |
| coreutils      | 36.288 s         | 24.8 MB/s  |
| fourmilab      | 103.160 s        | 8.7 MB/s   |

### Rust libb64 results (i7‑4510U)

| Operation | Time | Throughput |
|-----------|------|------------|
| encode+decode 50× | 11.55–12.07 s | ~76 MB/s |

### Summary

- Rust libb64: ~76 MB/s  
- C libb64‑1.2: 31.7 MB/s  
- coreutils base64: 24.8 MB/s  
- fourmilab base64: 8.7 MB/s  

The Rust implementation is faster than the original C version on a per‑core basis.
