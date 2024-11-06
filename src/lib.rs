// File: src/lib.rs
use std::ops::RangeInclusive;
#[cfg(all(target_arch = "x86_64", any(target_feature = "avx512f", target_feature = "avx2", target_feature = "aes")))]
use std::arch::x86_64::*;

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
                    size_range: 17..=usize::MAX,
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

    #[cfg(target_feature = "avx512f")]
    fn shard_with_avx512(&self, key: &[u8]) -> u32 {
        unsafe {
            if is_x86_feature_detected!("avx512f") {
                let mut hash = 0u32;
                for chunk in key.chunks(64) {
                    let vec = if chunk.len() == 64 {
                        _mm512_loadu_si512(chunk.as_ptr() as *const _)
                    } else {
                        let mut padded = [0u8; 64];
                        padded[..chunk.len()].copy_from_slice(chunk);
                        _mm512_loadu_si512(padded.as_ptr() as *const _)
                    };
                    
                    let reduced = _mm512_reduce_add_epi32(vec);
                    hash = hash.wrapping_add(reduced as u32);
                }
                hash % self.shard_count
            } else {
                self.shard_with_xxh3(key)
            }
        }
    }

    #[cfg(not(target_feature = "avx512f"))]
    fn shard_with_avx512(&self, key: &[u8]) -> u32 {
        self.shard_with_xxh3(key)
    }

    #[cfg(target_feature = "avx2")]
    fn shard_with_avx2(&self, key: &[u8]) -> u32 {
        unsafe {
            if is_x86_feature_detected!("avx2") {
                let mut hash = 0u32;
                for chunk in key.chunks(32) {
                    let vec = if chunk.len() == 32 {
                        _mm256_loadu_si256(chunk.as_ptr() as *const _)
                    } else {
                        let mut padded = [0u8; 32];
                        padded[..chunk.len()].copy_from_slice(chunk);
                        _mm256_loadu_si256(padded.as_ptr() as *const _)
                    };
                    
                    let reduced = _mm256_extract_epi32::<0>(vec) as u32;
                    hash = hash.wrapping_add(reduced);
                }
                hash % self.shard_count
            } else {
                self.shard_with_xxh3(key)
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    fn shard_with_avx2(&self, key: &[u8]) -> u32 {
        self.shard_with_xxh3(key)
    }

    #[cfg(target_feature = "aes")]
    fn shard_with_aesni(&self, key: &[u8]) -> u32 {
        unsafe {
            if is_x86_feature_detected!("aes") {
                let mut hash = _mm_set1_epi32(0);
                for chunk in key.chunks(16) {
                    let data = if chunk.len() == 16 {
                        _mm_loadu_si128(chunk.as_ptr() as *const _)
                    } else {
                        let mut padded = [0u8; 16];
                        padded[..chunk.len()].copy_from_slice(chunk);
                        _mm_loadu_si128(padded.as_ptr() as *const _)
                    };
                    
                    hash = _mm_aesenc_si128(hash, data);
                }
                let result = _mm_extract_epi32::<0>(hash) as u32;
                result % self.shard_count
            } else {
                self.shard_with_xxh3(key)
            }
        }
    }

    #[cfg(not(target_feature = "aes"))]
    fn shard_with_aesni(&self, key: &[u8]) -> u32 {
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

