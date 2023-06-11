use std::{path::Path, sync::Arc};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use forest_chain::ChainStore;
use forest_db::{
    db_engine::{db_root, open_proxy_db},
    parity_db_config::ParityDbConfig,
};
use forest_genesis::{import_chain, read_genesis_header, validate_chain};
use forest_networks::ChainConfig;
use forest_state_manager::StateManager;

fn bench_fibs(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let chain_data_root = Path::new("/home/aatif/.local/share/forest/calibnet");
    let snapshot_path =
        Path::new("/home/aatif/chainsafe/snapshots/filecoin_full_calibnet_2023-04-07_450000.car");
    let chain_config = Arc::new(ChainConfig::calibnet());

    let db = open_proxy_db(db_root(chain_data_root), ParityDbConfig::default()).unwrap();
    let genesis_header = runtime
        .block_on(read_genesis_header(
            None,
            Some(forest_networks::calibnet::DEFAULT_GENESIS),
            &db,
        ))
        .unwrap();

    // Initialize ChainStore
    let chain_store = Arc::new(
        ChainStore::new(db, chain_config.clone(), &genesis_header, chain_data_root).unwrap(),
    );
    chain_store.set_genesis(&genesis_header).unwrap();

    // Initialize StateManager
    let sm = Arc::new(
        StateManager::new(
            chain_store,
            chain_config,
            forest_fil_cns::composition::reward_calc(),
        )
        .unwrap(),
    );

    runtime
        .block_on(import_chain(
            &sm,
            snapshot_path.display().to_string().as_str(),
            true,
        ))
        .unwrap();

    let mut group = c.benchmark_group("Validate");
    for height in [
        -10, -20, -50, -100, -500, -1_000, /* -2_000, -5_000, -10_000, -20_000, */
    ] {
        let height = &height;
        group.bench_with_input(BenchmarkId::new("Validate", height), height, |b, height| {
            b.to_async(&runtime).iter(|| validate_chain(&sm, *height))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
