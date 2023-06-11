use forest_cli_shared::cli::CliOpts;
use forest_daemon::daemon;
use forest_networks::NetworkChain::Calibnet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = CliOpts {
        chain: Some(Calibnet),
        no_gc: true,
        import_snapshot: Some(String::from(
            "/home/aatif/chainsafe/snapshots/filecoin_full_calibnet_2023-04-07_450000.car",
        )),
        skip_load: Some(true),
        height: Some(-1000),
        halt_after_import: true,
        ..Default::default()
    };
    daemon::start(
        opts.clone(),
        opts.to_config().expect("this interface is silly").0,
        tokio::sync::mpsc::channel(1).0,
    )
    .await?;
    Ok(())
}
