// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::db::car::ManyCar;
use cid::Cid;
use fvm_ipld_blockstore::Blockstore;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct BlockstorePool<BS> {
    pool: RwLock<Vec<BS>>,
    fallback: BS,
}

impl<BS: Blockstore + Clone> BlockstorePool<BS> {
    pub fn new(bs: BS, pool_size: usize) -> Self {
        assert!(pool_size > 1, "pool size must be greater than 1");
        BlockstorePool {
            pool: RwLock::new(vec![bs.clone(); pool_size]),
            fallback: bs,
        }
    }
}

impl<BS: Blockstore> Blockstore for BlockstorePool<BS> {
    fn get(&self, k: &Cid) -> anyhow::Result<Option<Vec<u8>>> {
        let mut write_lock = self.pool.write();
        if let Some(bs) = write_lock.pop() {
            drop(write_lock);
            let result = bs.get(k);
            self.pool.write().push(bs);
            result
        } else {
            self.fallback.get(k)
        }
    }

    fn put_keyed(&self, k: &Cid, block: &[u8]) -> anyhow::Result<()> {
        let mut write_lock = self.pool.write();
        if let Some(bs) = write_lock.pop() {
            drop(write_lock);
            let result = bs.put_keyed(k, block);
            self.pool.write().push(bs);
            result
        } else {
            self.fallback.put_keyed(k, block)
        }
    }
}
