// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::{fs, path::PathBuf, sync::Arc, time::Duration};

use anyhow::bail;
use chrono::{Datelike, Utc};
use clap::Subcommand;
use dialoguer::{theme::ColorfulTheme, Confirm};
use forest_blocks::{tipset_keys_json::TipsetKeysJson, Tipset, TipsetKeys};
use forest_chain::ChainStore;
use forest_cli_shared::cli::{
    default_snapshot_dir, is_car_or_zst_or_tmp, snapshot_fetch, SnapshotServer, SnapshotStore,
};
use forest_db::db_engine::{db_root, open_proxy_db};
use forest_genesis::{forest_load_car, read_genesis_header};
use forest_ipld::{recurse_links_hash, CidHashSet};
use forest_networks::NetworkChain;
use forest_rpc_api::{chain_api::ChainExportParams, progress_api::GetProgressType};
use forest_rpc_client::{chain_ops::*, progress_ops::get_progress};
use forest_utils::{
    io::{parser::parse_duration, ProgressBar},
    net::get_fetch_progress_from_file,
    retry,
};
use fvm_shared::clock::ChainEpoch;
use log::info;
use strfmt::strfmt;
use tempfile::TempDir;
use tokio::time::sleep;

use super::*;
use crate::cli::{cli_error_and_die, handle_rpc_err};

pub(crate) const OUTPUT_PATH_DEFAULT_FORMAT: &str =
    "forest_snapshot_{chain}_{year}-{month}-{day}_height_{height}.car";

pub(crate) const OUTPUT_PATH_DEFAULT_COMPRESSED_FORMAT: &str =
    "forest_snapshot_{chain}_{year}-{month}-{day}_height_{height}.car.zst";

#[derive(Debug, Subcommand)]
pub enum SnapshotCommands {
    /// Export a snapshot of the chain to `<output_path>`
    Export {
        /// Snapshot output path. Default to
        /// `forest_snapshot_{chain}_{year}-{month}-{day}_height_{height}.car(.
        /// zst)` Date is in ISO 8601 date format.
        /// Arguments:
        ///  - chain - chain name e.g. `mainnet`
        ///  - year
        ///  - month
        ///  - day
        ///  - height - the epoch
        #[arg(short, default_value = ".", verbatim_doc_comment)]
        output_path: PathBuf,
        /// Export in zstd compressed format
        #[arg(long)]
        compressed: bool,
        /// Skip creating the checksum file. Only valid when `--compressed` is
        /// not supplied.
        #[arg(long)]
        skip_checksum: bool,
        /// Skip writing to the snapshot `.car` file specified by
        /// `--output-path`.
        #[arg(long)]
        dry_run: bool,
    },

    /// Fetches the most recent snapshot from a trusted, pre-defined location.
    Fetch {
        /// Directory to which the snapshot should be downloaded. If not
        /// provided, it will be saved in default Forest data location.
        #[arg(short, long)]
        snapshot_dir: Option<PathBuf>,
        /// Snapshot trusted source
        #[arg(short, long, value_enum)]
        provider: Option<SnapshotServer>,
        /// Download zstd compressed snapshot, only supported by the Filecoin
        /// provider for now. default is false.
        #[arg(long)]
        compressed: bool,
        /// Use [`aria2`](https://aria2.github.io/) for downloading, default is false. Requires `aria2c` in PATH.
        #[arg(long)]
        aria2: bool,
        /// Maximum number of times to retry the fetch
        #[arg(short, long, default_value = "3")]
        max_retries: i32,
        /// Duration to wait between the retries in seconds
        #[arg(short, long, default_value = "60", value_parser = parse_duration)]
        delay: Duration,
    },

    /// Shows default snapshot dir
    Dir,

    /// List local snapshots
    List {
        /// Directory to which the snapshots are downloaded. If not provided, it
        /// will be the default Forest data location.
        #[arg(short, long)]
        snapshot_dir: Option<PathBuf>,
    },

    /// Remove local snapshot
    Remove {
        /// Snapshot filename to remove
        filename: PathBuf,

        /// Directory to which the snapshots are downloaded. If not provided, it
        /// will be the default Forest data location.
        #[arg(short, long)]
        snapshot_dir: Option<PathBuf>,

        /// Answer yes to all forest-cli yes/no questions without prompting
        #[arg(long)]
        force: bool,
    },

    /// Prune local snapshot, keeps the latest only.
    /// Note that file names that do not match
    /// forest_snapshot_{chain}_{year}-{month}-{day}_height_{height}.car
    /// pattern will be ignored
    Prune {
        /// Directory to which the snapshots are downloaded. If not provided, it
        /// will be the default Forest data location.
        #[arg(short, long)]
        snapshot_dir: Option<PathBuf>,

        /// Answer yes to all forest-cli yes/no questions without prompting
        #[arg(long)]
        force: bool,
    },

    /// Clean all local snapshots, use with care.
    Clean {
        /// Directory to which the snapshots are downloaded. If not provided, it
        /// will be the default Forest data location.
        #[arg(short, long)]
        snapshot_dir: Option<PathBuf>,

        /// Answer yes to all forest-cli yes/no questions without prompting
        #[arg(long)]
        force: bool,
    },
    /// Validates the snapshot.
    Validate {
        /// Number of block headers to validate from the tip
        #[arg(long, default_value = "2000")]
        recent_stateroots: i64,
        /// Path to snapshot file
        snapshot: PathBuf,
        /// Force validation and answers yes to all prompts.
        #[arg(long)]
        force: bool,
    },
}

impl SnapshotCommands {
    pub async fn run(&self, config: Config) -> anyhow::Result<()> {
        match self {
            Self::Export {
                output_path,
                compressed,
                skip_checksum,
                dry_run,
            } => {
                let chain_head = match chain_head(&config.client.rpc_token).await {
                    Ok(head) => head.0,
                    Err(_) => cli_error_and_die("Could not get network head", 1),
                };

                let epoch = chain_head.epoch();

                let now = Utc::now();

                let month_string = format!("{:02}", now.month() as u8);
                let year = now.year();
                let day_string = format!("{:02}", now.day());
                let chain_name = chain_get_name((), &config.client.rpc_token)
                    .await
                    .map_err(handle_rpc_err)?;

                #[allow(clippy::disallowed_types)]
                let vars = std::collections::HashMap::from([
                    ("year".to_string(), year.to_string()),
                    ("month".to_string(), month_string),
                    ("day".to_string(), day_string),
                    ("chain".to_string(), chain_name),
                    ("height".to_string(), epoch.to_string()),
                ]);

                let output_path = if output_path.is_dir() {
                    output_path.join(if *compressed {
                        OUTPUT_PATH_DEFAULT_COMPRESSED_FORMAT
                    } else {
                        OUTPUT_PATH_DEFAULT_FORMAT
                    })
                } else {
                    output_path.clone()
                };

                let output_path = match strfmt(&output_path.display().to_string(), &vars) {
                    Ok(path) => path.into(),
                    Err(e) => {
                        cli_error_and_die(format!("Unparsable string error: {e}"), 1);
                    }
                };

                let params = ChainExportParams {
                    epoch,
                    recent_roots: config.chain.recent_state_roots,
                    output_path,
                    tipset_keys: TipsetKeysJson(chain_head.key().clone()),
                    compressed: *compressed,
                    skip_checksum: *skip_checksum,
                    dry_run: *dry_run,
                };

                let bar = Arc::new(tokio::sync::Mutex::new({
                    let bar = ProgressBar::new(0);
                    bar.message("Exporting snapshot | blocks ");
                    bar
                }));
                tokio::spawn({
                    let bar = bar.clone();
                    async move {
                        let mut interval =
                            tokio::time::interval(tokio::time::Duration::from_secs(1));
                        loop {
                            interval.tick().await;
                            if let Ok((progress, total)) =
                                get_progress((GetProgressType::SnapshotExport,), &None).await
                            {
                                let bar = bar.lock().await;
                                if bar.is_finish() {
                                    break;
                                }
                                bar.set_total(total);
                                bar.set(progress);
                            }
                        }
                    }
                });

                let out = chain_export(params, &config.client.rpc_token)
                    .await
                    .map_err(handle_rpc_err)?;

                bar.lock().await.finish_println(&format!(
                    "Export completed. Snapshot located at {}",
                    out.display()
                ));
                Ok(())
            }
            Self::Fetch {
                snapshot_dir,
                provider,
                compressed: use_compressed,
                aria2: use_aria2,
                max_retries,
                delay,
            } => {
                let snapshot_dir = snapshot_dir
                    .clone()
                    .unwrap_or_else(|| default_snapshot_dir(&config));
                match retry!(
                    snapshot_fetch,
                    *max_retries,
                    *delay,
                    &snapshot_dir,
                    &config,
                    provider,
                    *use_compressed,
                    *use_aria2
                ) {
                    Ok(out) => {
                        println!("Snapshot successfully downloaded at {}", out.display());
                        Ok(())
                    }
                    Err(e) => cli_error_and_die(format!("Failed fetching the snapshot: {e}"), 1),
                }
            }
            Self::Dir => {
                let dir = default_snapshot_dir(&config);
                println!("{}", dir.display());
                Ok(())
            }
            Self::List { snapshot_dir } => list(&config, snapshot_dir),
            Self::Remove {
                filename,
                snapshot_dir,
                force,
            } => {
                remove(&config, filename, snapshot_dir, *force);
                Ok(())
            }
            Self::Prune {
                snapshot_dir,
                force,
            } => {
                prune(&config, snapshot_dir, *force);
                Ok(())
            }
            Self::Clean {
                snapshot_dir,
                force,
            } => clean(&config, snapshot_dir, *force),
            Self::Validate {
                recent_stateroots,
                snapshot,
                force,
            } => validate(&config, recent_stateroots, snapshot, *force).await,
        }
    }
}

fn list(config: &Config, snapshot_dir: &Option<PathBuf>) -> anyhow::Result<()> {
    let snapshot_dir = snapshot_dir
        .clone()
        .unwrap_or_else(|| default_snapshot_dir(config));
    println!("Snapshot dir: {}", snapshot_dir.display());
    let store = SnapshotStore::new(config, &snapshot_dir);
    if store.snapshots.is_empty() {
        println!("No local snapshots");
    } else {
        println!("Local snapshots:");
        store.display();
    }
    Ok(())
}

fn remove(config: &Config, filename: &PathBuf, snapshot_dir: &Option<PathBuf>, force: bool) {
    let snapshot_dir = snapshot_dir
        .clone()
        .unwrap_or_else(|| default_snapshot_dir(config));
    let snapshot_path = snapshot_dir.join(filename);
    if snapshot_path.exists() && snapshot_path.is_file() && is_car_or_zst_or_tmp(&snapshot_path) {
        println!("Deleting {}", snapshot_path.display());
        if !force && !prompt_confirm() {
            println!("Aborted.");
            return;
        }

        delete_snapshot(&snapshot_path);
    } else {
        println!(
                "{} is not a valid snapshot file path, to list all snapshots, run forest-cli snapshot list",
                snapshot_path.display());
    }
}

fn prune(config: &Config, snapshot_dir: &Option<PathBuf>, force: bool) {
    let snapshot_dir = snapshot_dir
        .clone()
        .unwrap_or_else(|| default_snapshot_dir(config));
    println!("Snapshot dir: {}", snapshot_dir.display());
    let mut store = SnapshotStore::new(config, &snapshot_dir);
    if store.snapshots.len() < 2 {
        println!("No files to delete");
        return;
    }
    store.snapshots.sort_by_key(|s| (s.date, s.height));
    store.snapshots.pop(); // Keep the latest snapshot

    println!("Files to delete:");
    store.display();

    if !force && !prompt_confirm() {
        println!("Aborted.");
    } else {
        for snapshot_path in store.snapshots {
            delete_snapshot(&snapshot_path.path);
        }
    }
}

fn clean(config: &Config, snapshot_dir: &Option<PathBuf>, force: bool) -> anyhow::Result<()> {
    let snapshot_dir = snapshot_dir
        .clone()
        .unwrap_or_else(|| default_snapshot_dir(config));
    println!("Snapshot dir: {}", snapshot_dir.display());

    let read_dir = match fs::read_dir(snapshot_dir) {
        Ok(read_dir) => read_dir,
        // basically have the same behavior as in `rm -f` which doesn't fail if the target
        // directory doesn't exist.
        Err(_) if force => {
            println!("Target directory not accessible. Skipping.");
            return Ok(());
        }
        Err(e) => bail!(e),
    };

    let snapshots_to_delete: Vec<_> = read_dir
        .flatten()
        .map(|entry| entry.path())
        .filter(|p| is_car_or_zst_or_tmp(p))
        .collect();

    if snapshots_to_delete.is_empty() {
        println!("No files to delete");
        return Ok(());
    }
    println!("Files to delete:");
    snapshots_to_delete
        .iter()
        .for_each(|f| println!("{}", f.display()));

    if !force && !prompt_confirm() {
        println!("Aborted.");
        return Ok(());
    }

    for snapshot_path in snapshots_to_delete {
        delete_snapshot(&snapshot_path);
    }

    Ok(())
}

async fn validate(
    config: &Config,
    recent_stateroots: &i64,
    snapshot: &PathBuf,
    force: bool,
) -> anyhow::Result<()> {
    let confirm = force
        || atty::is(atty::Stream::Stdin)
            && Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "This will result in using approximately {} MB of data. Proceed?",
                    std::fs::metadata(snapshot)?.len() / (1024 * 1024)
                ))
                .default(false)
                .interact()
                .unwrap_or_default();

    if confirm {
        let tmp_chain_data_path = TempDir::new()?;
        let db_path = db_root(
            tmp_chain_data_path
                .path()
                .join(config.chain.network.to_string())
                .as_path(),
        );
        let db = open_proxy_db(db_path, config.db_config().clone())?;

        let genesis = read_genesis_header(
            config.client.genesis_file.as_ref(),
            config.chain.genesis_bytes(),
            &db,
        )
        .await?;

        let chain_store = Arc::new(ChainStore::new(
            db,
            config.chain.clone(),
            &genesis,
            tmp_chain_data_path.path(),
        )?);

        let (cids, _n_records) = {
            let reader = get_fetch_progress_from_file(&snapshot).await?;
            forest_load_car(chain_store.blockstore().clone(), reader).await?
        };

        let ts = chain_store.tipset_from_keys(&TipsetKeys::new(cids))?;

        validate_links_and_genesis_traversal(
            &chain_store,
            ts,
            chain_store.blockstore(),
            *recent_stateroots,
            &Tipset::from(genesis),
            &config.chain.network,
        )
        .await?;
    }

    Ok(())
}

async fn validate_links_and_genesis_traversal<DB>(
    chain_store: &ChainStore<DB>,
    ts: Arc<Tipset>,
    db: &DB,
    recent_stateroots: ChainEpoch,
    genesis_tipset: &Tipset,
    network: &NetworkChain,
) -> anyhow::Result<()>
where
    DB: fvm_ipld_blockstore::Blockstore + Send + Sync,
{
    let mut seen = CidHashSet::default();
    let upto = ts.epoch() - recent_stateroots;

    let mut tsk = ts.parents().clone();

    let total_size = ts.epoch();
    let pb = forest_utils::io::ProgressBar::new(total_size as u64);
    pb.message("Validating tipsets: ");
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    // Security: Recursive snapshots are difficult to create but not impossible.
    // This limits the amount of recursion we do.
    let mut prev_epoch = ts.epoch();
    loop {
        // if we reach 0 here, it means parent traversal didn't end up reaching genesis
        // properly, bail with error.
        if prev_epoch <= 0 {
            bail!("Broken invariant: no genesis tipset in snapshot.");
        }

        let tipset = chain_store.tipset_from_keys(&tsk)?;
        let height = tipset.epoch();
        // if parent tipset epoch is smaller than child, bail with error.
        if height >= prev_epoch {
            bail!("Broken tipset invariant: parent epoch larger than current epoch at: {height}");
        }
        // genesis is reachable, break with success
        if height == 0 {
            if tipset.as_ref() != genesis_tipset {
                bail!("Invalid genesis tipset. Snapshot isn't valid for {network}. It may be valid for another network.");
            }

            break;
        }
        // check for ipld links backwards till `upto`
        if height > upto {
            let mut assert_cid_exists = |cid: Cid| async move {
                let data = db.get(&cid)?;
                data.ok_or_else(|| anyhow::anyhow!("Broken IPLD link at epoch: {height}"))
            };

            for h in tipset.blocks() {
                recurse_links_hash(&mut seen, *h.state_root(), &mut assert_cid_exists, &|_| ())
                    .await?;
                recurse_links_hash(&mut seen, *h.messages(), &mut assert_cid_exists, &|_| ())
                    .await?;
            }
        }

        tsk = tipset.parents().clone();
        prev_epoch = tipset.epoch();
        pb.set((ts.epoch() - tipset.epoch()) as u64);
    }

    drop(pb);

    println!("Snapshot is valid");

    Ok(())
}

fn delete_snapshot(snapshot_path: &PathBuf) {
    let checksum_path = snapshot_path.with_extension("sha256sum");
    for path in [snapshot_path, &checksum_path] {
        if path.exists() {
            if let Err(err) = fs::remove_file(path) {
                println!("Failed to delete {}\n{err}", path.display());
            } else {
                println!("Deleted {}", path.display());
            }
        }
    }
}
