use std::{path::Path, sync::Arc};

use anyhow::Context;
use forest_chain::ChainStore;
use forest_db::{
    db_engine::{db_root, open_proxy_db},
    parity_db_config::ParityDbConfig,
};
use forest_genesis::{forest_load_car, import_chain, read_genesis_header, validate_chain};
use forest_networks::{ChainConfig, Height, HeightInfo};
use forest_state_manager::StateManager;
use fvm_shared::clock::ChainEpoch;
use tracing::info;

async fn do_validate_chain(
    chain_data_root: &Path,
    snapshot_path: &Path,
    height: i64,
) -> anyhow::Result<()> {
    forest_shim::address::set_current_network(forest_shim::address::Network::Testnet);
    dbg!("here");
    let chain_config = Arc::new(ChainConfig::calibnet());
    dbg!("here");
    let db = open_proxy_db(db_root(chain_data_root), ParityDbConfig::default()).unwrap();
    dbg!("here");
    let genesis_header =
        read_genesis_header(None, Some(forest_networks::calibnet::DEFAULT_GENESIS), &db)
            .await
            .context("couldn't read genesis header")?;

    // Initialize ChainStore
    dbg!("here");
    let chain_store = Arc::new(
        ChainStore::new(db, chain_config.clone(), &genesis_header, chain_data_root).unwrap(),
    );
    dbg!("here");
    chain_store
        .set_genesis(&genesis_header)
        .context("couldn't set genesis header")?;

    // Initialize StateManager
    dbg!("here");
    let sm = Arc::new(
        StateManager::new(
            chain_store,
            chain_config,
            forest_fil_cns::composition::reward_calc(),
        )
        .context("couldnt' initialize state manager")?,
    );

    dbg!("here");
    import_chain(&sm, snapshot_path.display().to_string().as_str(), true)
        .await
        .context("couldn't import chain")?;

    dbg!("here");
    validate_chain(&sm, height)
        .await
        .context("couldn't validate chain")?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chain_data_root = Path::new("/home/aatif/.local/share/forest/calibnet");
    let snapshot_path =
        Path::new("/home/aatif/chainsafe/snapshots/filecoin_full_calibnet_2023-04-07_450000.car");
    // 450000 // height
    // failure heights:
    // 449802
    // 449882
    // 449874
    // 449920
    // I get a different failure whenver I enter a previously failed height??
    do_validate_chain(chain_data_root, snapshot_path, 449882).await?;
    Ok(())
}
