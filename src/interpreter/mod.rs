// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod errors;
mod fvm;
pub mod fvm3;
#[cfg(feature = "instrumented_kernel")]
mod instrumented_kernel;
#[cfg(feature = "instrumented_kernel")]
mod metrics;
mod vm;

use std::sync::Arc;

use crate::{
    networks::ChainConfig,
    shim::{
        address::{Address, Protocol},
        state_tree::StateTree,
    },
};
use anyhow::bail;
use cid::Cid;
use fil_actor_interface::account;
use fvm_ipld_blockstore::{tracking::TrackingBlockstore, Blockstore};
use fvm_shared::clock::ChainEpoch;

pub use self::vm::*;

fn do_worker_key_at_lookback<DB: Blockstore>(
    miner_addr: &Address,
    height: ChainEpoch,
    // rand: Box<dyn Rand>,
    epoch: ChainEpoch,
    root: Cid,
    lookback: &dyn Fn(ChainEpoch) -> anyhow::Result<Cid>,
    // get_tsk: Box<dyn Fn(ChainEpoch) -> anyhow::Result<crate::blocks::TipsetKeys>>,
    db: DB,
    chain_config: &Arc<ChainConfig>,
    // bail: AtomicBool,
) -> anyhow::Result<(Address /*, i64*/,)> {
    if height < epoch - chain_config.policy.chain_finality {
        bail!(
            "cannot get worker key (current epoch: {}, height: {})",
            epoch,
            height
        );
    }

    let prev_root = (lookback)(height)?;
    let lb_state = StateTree::new_from_root(&db, &prev_root)?;

    let actor = lb_state
        .get_actor(miner_addr)?
        .ok_or_else(|| anyhow::anyhow!("actor not found {:?}", miner_addr))?;

    let tbs = TrackingBlockstore::new(&db);

    let ms = fil_actor_interface::miner::State::load(&tbs, actor.code, actor.state)?;

    let worker = ms.info(&tbs)?.worker;

    let state = StateTree::new_from_root(&db, &root)?;

    let addr = resolve_to_key_addr(&state, &tbs, &worker.into())?;

    // let network_version = chain_config.network_version(epoch);
    // let gas_used = cal_gas_used_from_stats(tbs.stats.borrow(), network_version)?;

    Ok((addr /*gas_used.round_up() as i64*/,))
}

/// returns the public key type of address (`BLS`/`SECP256K1`) of an account
/// actor identified by `addr`.
pub fn resolve_to_key_addr<BS, S>(
    st: &StateTree<S>,
    store: &BS,
    addr: &Address,
) -> Result<Address, anyhow::Error>
where
    BS: Blockstore,
    S: Blockstore + Clone,
{
    if addr.protocol() == Protocol::BLS
        || addr.protocol() == Protocol::Secp256k1
        || addr.protocol() == Protocol::Delegated
    {
        return Ok(*addr);
    }

    let act = st
        .get_actor(addr)?
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve actor: {}", addr))?;

    // If there _is_ an f4 address, return it as "key" address
    if let Some(address) = act.delegated_address {
        return Ok(address.into());
    }

    let acc_st = account::State::load(store, act.code, act.state)?;

    Ok(acc_st.pubkey_address().into())
}
