use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use fast_shard::{FastShard, ShardConfig, ShardTier, ShardAlgorithm};

fn create_single_algo_config(algo: ShardAlgorithm) -> ShardConfig {
    ShardConfig {
        tiers: vec![
            ShardTier {
                size_range: 0..=usize::MAX,
                algorithms: vec![algo.clone()],
            },
        ],
        default_algorithms: vec![algo],
    }
}

pub fn bench_hash_algorithms(c: &mut Criterion) {
    let test_sizes = vec![
        4,    // 4 bytes
        8,    // 8 bytes
        16,   // 16 bytes
        32,   // 32 bytes
        256,  // 256 bytes
        512,  // 512 bytes
        1024, // 1KB
        4096, // 4KB
        32768 // 32KB
    ];
    let algorithms = vec![
        ("AVX512", ShardAlgorithm::Avx512),
        ("AVX2", ShardAlgorithm::Avx2),
        ("AES-NI", ShardAlgorithm::AesNi),
        ("XXH3", ShardAlgorithm::Xxh3),
        ("FNV1a", ShardAlgorithm::Fnv1a),
    ];

    let mut group = c.benchmark_group("hash_comparison");
    
    for &size in &test_sizes {
        let test_data = vec![0xAA; size]; // Create test data filled with 0xAA
        
        for (algo_name, algo) in &algorithms {
            let config = create_single_algo_config(algo.clone());
            let shard = FastShard::with_config(1024, config);
            
            group.bench_with_input(
                BenchmarkId::new(*algo_name, size),
                &test_data,
                |b, data| {
                    b.iter(|| shard.shard(data));
                },
            );
        }
    }
    
    group.finish();
}

criterion_group!(benches, bench_hash_algorithms);
criterion_main!(benches);
