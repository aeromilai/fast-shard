// File: src/lib.rs
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub enum ShardAlgorithm {
    Avx512,
    Avx2,
    AesNi,
    Fnv1a,
    Xxh3,
}

#[derive(Debug, Clone)]
pub struct ShardTier {
    pub size_range: RangeInclusive<usize>,
    pub algorithms: Vec<ShardAlgorithm>,
}

#[derive(Debug, Clone)]
pub struct ShardConfig {
    pub tiers: Vec<ShardTier>,
    pub default_algorithms: Vec<ShardAlgorithm>,
}

impl Default for ShardConfig {
    fn default() -> Self {
        let small_key_algorithms = vec![
            ShardAlgorithm::Avx512,
            ShardAlgorithm::Avx2,
            ShardAlgorithm::AesNi,
            ShardAlgorithm::Fnv1a,
            ShardAlgorithm::Xxh3,
        ];

        let large_key_algorithms = vec![
            ShardAlgorithm::Avx512,
            ShardAlgorithm::Avx2,
            ShardAlgorithm::AesNi,
            ShardAlgorithm::Xxh3,
            ShardAlgorithm::Fnv1a,
        ];

        ShardConfig {
            tiers: vec![
                ShardTier {
                    size_range: 0..=16,
                    algorithms: small_key_algorithms,
                },
                ShardTier {
                    size_range: 17..usize::MAX,
                    algorithms: large_key_algorithms,
                },
            ],
            default_algorithms: vec![ShardAlgorithm::Xxh3],
        }
    }
}

#[derive(Debug)]
pub struct FastShard {
    shard_count: u32,
    config: ShardConfig,
}

impl FastShard {
    pub fn new(shard_count: u32) -> Self {
        Self {
            shard_count,
            config: ShardConfig::default(),
        }
    }

    pub fn with_config(shard_count: u32, config: ShardConfig) -> Self {
        Self { shard_count, config }
    }

    fn get_available_algorithm(&self, algorithms: &[ShardAlgorithm]) -> ShardAlgorithm {
        for algo in algorithms {
            match algo {
                ShardAlgorithm::Avx512 => {
                    #[cfg(target_feature = "avx512f")]
                    return ShardAlgorithm::Avx512;
                }
                ShardAlgorithm::Avx2 => {
                    #[cfg(target_feature = "avx2")]
                    return ShardAlgorithm::Avx2;
                }
                ShardAlgorithm::AesNi => {
                    #[cfg(target_feature = "aes")]
                    return ShardAlgorithm::AesNi;
                }
                ShardAlgorithm::Fnv1a => return ShardAlgorithm::Fnv1a,
                ShardAlgorithm::Xxh3 => return ShardAlgorithm::Xxh3,
            }
        }
        ShardAlgorithm::Xxh3 // Final fallback
    }

    fn get_algorithm_for_size(&self, size: usize) -> ShardAlgorithm {
        for tier in &self.config.tiers {
            if tier.size_range.contains(&size) {
                return self.get_available_algorithm(&tier.algorithms);
            }
        }
        self.get_available_algorithm(&self.config.default_algorithms)
    }

    pub fn shard(&self, key: &[u8]) -> u32 {
        let algorithm = self.get_algorithm_for_size(key.len());
        match algorithm {
            ShardAlgorithm::Avx512 => self.shard_with_avx512(key),
            ShardAlgorithm::Avx2 => self.shard_with_avx2(key),
            ShardAlgorithm::AesNi => self.shard_with_aesni(key),
            ShardAlgorithm::Fnv1a => self.shard_with_fnv1a(key),
            ShardAlgorithm::Xxh3 => self.shard_with_xxh3(key),
        }
    }

    fn shard_with_avx512(&self, key: &[u8]) -> u32 {
        // TODO: Implement AVX-512 sharding
        self.shard_with_xxh3(key)
    }

    fn shard_with_avx2(&self, key: &[u8]) -> u32 {
        // TODO: Implement AVX2 sharding
        self.shard_with_xxh3(key)
    }

    fn shard_with_aesni(&self, key: &[u8]) -> u32 {
        // TODO: Implement AES-NI sharding
        self.shard_with_xxh3(key)
    }

    fn shard_with_fnv1a(&self, key: &[u8]) -> u32 {
        let mut hasher = fnv::FnvHasher::default();
        use std::hash::Hasher;
        hasher.write(key);
        (hasher.finish() % self.shard_count as u64) as u32
    }

    fn shard_with_xxh3(&self, key: &[u8]) -> u32 {
        use xxhash_rust::xxh3::xxh3_64;
        (xxh3_64(key) % self.shard_count as u64) as u32
    }
}

// Example usage
fn main() {
    // Define custom configuration
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
            ShardTier {
                size_range: 1025..=4096,
                algorithms: vec![
                    ShardAlgorithm::Avx512,
                    ShardAlgorithm::AesNi,
                    ShardAlgorithm::Xxh3,
                ],
            },
        ],
        default_algorithms: vec![
            ShardAlgorithm::Xxh3,
            ShardAlgorithm::Fnv1a,
        ],
    };

    let shard = FastShard::with_config(1024, config);
    
    // Use the configured sharding
    let small_key = b"small key";
    let medium_key = vec![0u8; 500];
    let large_key = vec![0u8; 2000];
    
    println!("Small key shard: {}", shard.shard(small_key));
    println!("Medium key shard: {}", shard.shard(&medium_key));
    println!("Large key shard: {}", shard.shard(&large_key));
}

// Add test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_config() {
        let config = ShardConfig {
            tiers: vec![
                ShardTier {
                    size_range: 0..=16,
                    algorithms: vec![ShardAlgorithm::Fnv1a],
                },
                ShardTier {
                    size_range: 17..=1024,
                    algorithms: vec![ShardAlgorithm::Xxh3],
                },
            ],
            default_algorithms: vec![ShardAlgorithm::Xxh3],
        };

        let shard = FastShard::with_config(16, config);
        
        let small_key = b"small";
        let large_key = vec![0u8; 100];
        
        // These should execute without panicking
        let _ = shard.shard(small_key);
        let _ = shard.shard(&large_key);
    }

    #[test]
    fn test_default_config() {
        let shard = FastShard::new(16);
        
        // Test various key sizes
        let keys = vec![
            vec![0u8; 8],    // Small
            vec![0u8; 16],   // Border
            vec![0u8; 32],   // Medium
            vec![0u8; 1024], // Large
        ];
        
        for key in keys {
            let _ = shard.shard(&key);
        }
    }
}

