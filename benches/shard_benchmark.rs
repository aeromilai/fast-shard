use criterion::{criterion_group, criterion_main, Criterion};
use fast_shard::{fast_shard, ShardConfig, ShardTier, ShardAlgorithm};

pub fn bench_configured_sharding(c: &mut Criterion) {
    let default_shard = fast_shard::new(1024);
    
    let custom_config = ShardConfig {
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
    
    let custom_shard = fast_shard::with_config(1024, custom_config);
    
    let small_key = vec![0u8; 32];
    let large_key = vec![0u8; 512];
    
    c.bench_function("default_small", |b| {
        b.iter(|| default_shard.shard(&small_key))
    });
    
    c.bench_function("custom_small", |b| {
        b.iter(|| custom_shard.shard(&small_key))
    });
    
    c.bench_function("default_large", |b| {
        b.iter(|| default_shard.shard(&large_key))
    });
    
    c.bench_function("custom_large", |b| {
        b.iter(|| custom_shard.shard(&large_key))
    });
}

criterion_group!(benches, bench_configured_sharding);
criterion_main!(benches);
