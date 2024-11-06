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

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.
