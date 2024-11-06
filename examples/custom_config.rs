use fast_shard::{FastShard, ShardConfig, ShardTier, ShardAlgorithm};

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
