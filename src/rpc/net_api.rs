// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::str::FromStr;
use std::time::Duration;

use crate::libp2p::chain_exchange::ChainExchangeRequest;
use crate::libp2p::{NetRPCMethods, NetworkMessage, PeerId};
use crate::rpc_api::{
    data_types::{AddrInfo, RPCState},
    net_api::*,
};
use cid::multibase;
use futures::channel::oneshot;
use fvm_ipld_blockstore::Blockstore;
use jsonrpc_v2::{Data, Error as JsonRpcError, Params};
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::error;

pub(in crate::rpc) async fn net_addrs_listen<DB: Blockstore + Clone + Send + Sync + 'static>(
    data: Data<RPCState<DB>>,
) -> Result<NetAddrsListenResult, JsonRpcError> {
    let (tx, rx) = oneshot::channel();
    let req = NetworkMessage::JSONRPCRequest {
        method: NetRPCMethods::AddrsListen(tx),
    };

    data.network_send.send_async(req).await?;
    let (id, addrs) = rx.await?;

    Ok(AddrInfo {
        id: id.to_string(),
        addrs,
    })
}

pub(in crate::rpc) async fn net_peers<DB: Blockstore + Clone + Send + Sync + 'static>(
    data: Data<RPCState<DB>>,
) -> Result<NetPeersResult, JsonRpcError> {
    let (tx, rx) = oneshot::channel();
    let req = NetworkMessage::JSONRPCRequest {
        method: NetRPCMethods::Peers(tx),
    };

    data.network_send.send_async(req).await?;
    let peer_addresses = rx.await?;

    let connections = peer_addresses
        .into_iter()
        .map(|(id, addrs)| AddrInfo {
            id: id.to_string(),
            addrs,
        })
        .collect();

    Ok(connections)
}

pub(in crate::rpc) async fn net_connect<DB: Blockstore + Clone + Send + Sync + 'static>(
    data: Data<RPCState<DB>>,
    Params(params): Params<NetConnectParams>,
) -> Result<NetConnectResult, JsonRpcError> {
    let (AddrInfo { id, addrs },) = params;
    let (_, id) = multibase::decode(format!("{}{}", "z", id))?;
    let peer_id = PeerId::from_bytes(&id)?;

    let (tx, rx) = oneshot::channel();
    let req = NetworkMessage::JSONRPCRequest {
        method: NetRPCMethods::Connect(tx, peer_id, addrs),
    };

    data.network_send.send_async(req).await?;
    let success = rx.await?;

    if success {
        Ok(())
    } else {
        error!("Peer could not be dialed from any address provided");
        Err(JsonRpcError::INTERNAL_ERROR)
    }
}

pub(in crate::rpc) async fn net_disconnect<DB: Blockstore + Clone + Send + Sync + 'static>(
    data: Data<RPCState<DB>>,
    Params(params): Params<NetDisconnectParams>,
) -> Result<NetDisconnectResult, JsonRpcError> {
    let (id,) = params;
    let peer_id = PeerId::from_str(&id)?;

    let (tx, rx) = oneshot::channel();
    let req = NetworkMessage::JSONRPCRequest {
        method: NetRPCMethods::Disconnect(tx, peer_id),
    };

    data.network_send.send_async(req).await?;
    rx.await?;

    Ok(())
}

pub(in crate::rpc) async fn net_query<DB: Blockstore + Clone + Send + Sync + 'static>(
    data: Data<RPCState<DB>>,
    Params(params): Params<NetQueryParams>,
) -> Result<NetDisconnectResult, JsonRpcError> {
    let id = params;
    let peer_id = PeerId::from_str(&id)?;

    const MAX_CONCURRENT_REQUESTS: usize = 200;
    const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    let mut task_set = JoinSet::new();

    // iterate over all tipsets from heaviest to genesis
    let mut heaviest = data.chain_store.heaviest_tipset();
    let mut counter = 0;
    let request_len = 50;
    while let Ok(tipset) = data.chain_store.tipset_from_keys(heaviest.parents()) {
        let data = data.0.clone();
        heaviest = tipset.clone();
        let tipset = tipset.clone();

        if task_set.len() >= MAX_CONCURRENT_REQUESTS {
            let _ignore = task_set.join_next().await;
        }

        if counter % request_len == 0 {
            sleep(Duration::from_millis(10)).await;
            println!(
                "requested chain exchanges: {counter}. Current epoch {epoch}",
                counter = counter,
                epoch = heaviest.epoch()
            );
            task_set.spawn_blocking({
                let network_send = data.network_send.clone();
                move || {
                    let (tx, rx) = flume::bounded(1);
                    let request = NetworkMessage::ChainExchangeRequest {
                        peer_id,
                        request: ChainExchangeRequest {
                            start: tipset.key().cids().to_vec(),
                            request_len, //finality
                            options: 1,
                        },
                        response_channel: tx,
                    };

                    network_send.send(request).unwrap();
                    let _ignore = rx.recv_timeout(REQUEST_TIMEOUT);
                }
            });
        }
        counter += 1;
    }
    tokio::task::yield_now().await;

    while let Some(_ignore) = task_set.join_next().await {}

    //let cid = cid.try_into().unwrap();

    //data.network_send
    //    .send(NetworkMessage::BitswapRequestSinglePeer { cid, peer: peer_id })
    //    .unwrap();

    Ok(())
}
