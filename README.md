# fast-shard

[![Crates.io](https://img.shields.io/crates/v/fast-shard.svg)](https://crates.io/crates/fast-shard)
[![Documentation](https://docs.rs/fast-shard/badge.svg)](https://docs.rs/fast-shard)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A high-performance, configurable sharding library that leverages CPU vector instructions (AVX-512, AVX2, AES-NI) and efficient hashing algorithms (XXH3, FNV1a) for optimal performance across different key sizes.

## Features

- 🚀 Multiple sharding algorithms optimized for different key sizes
- ⚡ Hardware acceleration using AVX-512, AVX2, and AES-NI instructions
- 🔧 Fully configurable algorithm selection based on key sizes
- 📊 Built-in benchmarking suite
- 🛡️ Comprehensive test coverage
- 🔄 Automatic fallback to best available algorithm

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
fast-shard = "0.1.0"
```

Basic usage:
```rust
use fast_shard::FastShard;

// Create a sharding instance with default configuration
let shard = FastShard::new(1024); // 1024 shards

// Shard some data
let key = b"example key";
let shard_number = shard.shard(key);
```

## Performance

Default algorithm selection by key size:

| Key Size    | Algorithm Priority                           |
|------------|---------------------------------------------|
| ≤16 bytes  | AVX512 → AVX2 → AES-NI → FNV1a → XXH3      |
| >16 bytes  | AVX512 → AVX2 → AES-NI → XXH3 → FNV1a      |

## Custom Configuration

Create custom sharding configurations based on your specific needs:

```rust
use fast_shard::{FastShard, ShardConfig, ShardTier, ShardAlgorithm};

let config = ShardConfig {
    tiers: vec![
        ShardTier {
            size_range: 0..=128,
            algorithms: vec![
                ShardAlgorithm::Avx512,
                ShardAlgorithm::AesNi,
                ShardAlgorithm::Fnv1a,
            ],
        },
        ShardTier {
            size_range: 129..=1024,
            algorithms: vec![
                ShardAlgorithm::Avx512,
                ShardAlgorithm::Avx2,
                ShardAlgorithm::Xxh3,
            ],
        },
    ],
    default_algorithms: vec![ShardAlgorithm::Xxh3],
};

let shard = FastShard::with_config(1024, config);
```

## Feature Flags

- `nightly` - Enable nightly features (required for AVX-512)
- `runtime-detection` - Enable runtime CPU feature detection
- `std` - Standard library support (enabled by default)

## CPU Feature Requirements

For optimal performance, enable CPU features in your `.cargo/config.toml`:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

Or enable specific features:
```toml
[build]
rustflags = [
    "-C", "target-feature=+avx2,+aes"
]
```

## Benchmarking

Run the benchmark suite:
```bash
cargo bench
```

## Examples

### Basic Usage
```rust
use fast_shard::FastShard;

let shard = FastShard::new(1024);
let key = b"example key";
println!("Shard: {}", shard.shard(key));
```

### With Custom Configuration
```rust
use fast_shard::{FastShard, ShardConfig, ShardTier, ShardAlgorithm};

// Configure for specific key size ranges
let config = ShardConfig {
    tiers: vec![
        ShardTier {
            size_range: 0..=64,
            algorithms: vec![ShardAlgorithm::Fnv1a],
        },
        ShardTier {
            size_range: 65..=1024,
            algorithms: vec![ShardAlgorithm::Xxh3],
        },
    ],
    default_algorithms: vec![ShardAlgorithm::Xxh3],
};

let shard = FastShard::with_config(1024, config);
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Credits

This crate uses the following high-quality dependencies:
- [xxhash-rust](https://github.com/DoumanAsh/xxhash-rust) for XXH3 implementation
- [fnv](https://github.com/servo/rust-fnv) for FNV-1a implementation

## Safety

This crate uses `unsafe` code for SIMD operations but maintains safety through:
- Careful bounds checking
- Extensive testing including property-based tests
- Runtime CPU feature detection when enabled
- Proper alignment handling

##  Benchmark
```
sample benchmarks by running the following: 
cargo bench --bench hash_comparison

time:   [2.3172 ns 2.3432 ns 2.3756 ns]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
hash_comparison/AVX2/4  time:   [2.2711 ns 2.2934 ns 2.3242 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
hash_comparison/AES-NI/4
                        time:   [2.2902 ns 2.3002 ns 2.3111 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
hash_comparison/XXH3/4  time:   [2.2938 ns 2.3060 ns 2.3191 ns]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
hash_comparison/FNV1a/4 time:   [2.4171 ns 2.4227 ns 2.4294 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
hash_comparison/AVX512/8
                        time:   [2.3122 ns 2.3411 ns 2.3753 ns]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
hash_comparison/AVX2/8  time:   [2.3110 ns 2.3273 ns 2.3462 ns]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
hash_comparison/AES-NI/8
                        time:   [2.2988 ns 2.3104 ns 2.3245 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
hash_comparison/XXH3/8  time:   [2.2962 ns 2.3069 ns 2.3183 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
hash_comparison/FNV1a/8 time:   [3.1421 ns 3.1637 ns 3.1879 ns]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.
