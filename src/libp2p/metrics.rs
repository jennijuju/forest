// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use once_cell::sync::Lazy;
use prometheus::core::{AtomicU64, GenericCounter, GenericGauge};

pub static PEER_FAILURE_TOTAL: Lazy<Box<GenericCounter<AtomicU64>>> = Lazy::new(|| {
    let peer_failure_total = Box::new(
        GenericCounter::<AtomicU64>::new(
            "peer_failure_total",
            "Total number of failed peer requests",
        )
        .expect("Defining the peer_failure_total metric must succeed"),
    );
    prometheus::default_registry()
        .register(peer_failure_total.clone())
        .expect("Registering the peer_failure_total metric with the metrics registry must succeed");
    peer_failure_total
});
pub static FULL_PEERS: Lazy<Box<GenericGauge<AtomicU64>>> = Lazy::new(|| {
    let full_peers = Box::new(
        GenericGauge::<AtomicU64>::new(
            "full_peers",
            "Number of healthy peers recognized by the node",
        )
        .expect("Defining the full_peers metric must succeed"),
    );
    prometheus::default_registry()
        .register(full_peers.clone())
        .expect("Registering the full_peers metric with the metrics registry must succeed");
    full_peers
});
pub static BAD_PEERS: Lazy<Box<GenericGauge<AtomicU64>>> = Lazy::new(|| {
    let bad_peers = Box::new(
        GenericGauge::<AtomicU64>::new("bad_peers", "Number of bad peers recognized by the node")
            .expect("Defining the bad_peers metric must succeed"),
    );
    prometheus::default_registry()
        .register(bad_peers.clone())
        .expect("Registering the bad_peers metric with the metrics registry must succeed");
    bad_peers
});
