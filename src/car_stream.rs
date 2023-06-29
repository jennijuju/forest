// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::io::{self, ErrorKind::InvalidData};

use bytes::{Buf as _, Bytes, BytesMut};
use cid::Cid;
use futures::TryStreamExt as _;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_car::CarHeader;
use std::pin::pin;
use tokio::io::AsyncRead;
use tokio_util_06::codec::FramedRead;
use unsigned_varint::codec::UviBytes;

#[tracing::instrument(skip_all, ret, err)]
pub async fn load_car_into_blockstore(
    car: impl AsyncRead,
    blockstore: &impl Blockstore,
) -> anyhow::Result<CarHeader> {
    let mut varint_frames = pin!(FramedRead::new(car, UviBytes::<Bytes>::default()));
    let first_varint_frame = varint_frames
        .try_next()
        .await?
        .ok_or(io::Error::new(InvalidData, "CAR must start with a header"))?;
    let header = fvm_ipld_encoding::from_slice(&first_varint_frame)
        .map_err(|e| io::Error::new(InvalidData, e))?;
    varint_frames
        .err_into()
        .and_then(|varint_frame| async { cid_and_data(varint_frame) })
        // Batch our writes for efficiency with the underlying blockstore
        .try_chunks(100)
        .err_into()
        .try_for_each_concurrent(
            None,
            |chunk| async move { blockstore.put_many_keyed(chunk) },
        )
        .await?;
    Ok(header)
}

#[tracing::instrument(level = "trace", skip_all, err)]
fn cid_and_data(concatenated: BytesMut) -> anyhow::Result<(Cid, BytesMut)> {
    let mut reader = concatenated.reader();
    let cid = Cid::read_bytes(&mut reader)?;
    Ok((cid, reader.into_inner()))
}

#[cfg(test)]
mod tests {
    use super::load_car_into_blockstore;

    use ahash::HashMap;
    use cid::Cid;
    use futures::executor::block_on;
    use fvm_ipld_blockstore::Blockstore;
    use fvm_ipld_car::CarReader;
    use std::cell::RefCell;

    #[derive(Debug, Default, PartialEq)]
    struct TestBlockstore {
        pub inner: RefCell<HashMap<Cid, Vec<u8>>>,
    }

    impl Blockstore for TestBlockstore {
        fn get(&self, k: &Cid) -> anyhow::Result<Option<Vec<u8>>> {
            Ok(self.inner.borrow().get(k).cloned())
        }

        fn put_keyed(&self, k: &Cid, block: &[u8]) -> anyhow::Result<()> {
            self.inner
                .borrow_mut()
                .entry(*k)
                .and_modify(|already| assert_eq!(already, block))
                .or_insert(block.to_vec());
            Ok(())
        }
    }

    #[test]
    fn test() {
        let car = include_bytes!("../test-snapshots/chain4.car");

        let actual_store = TestBlockstore::default();
        let actual_header = block_on(async {
            load_car_into_blockstore(std::io::Cursor::new(car), &actual_store)
                .await
                .unwrap()
        });

        let expected_store = TestBlockstore::default();
        let expected_header = block_on(async {
            let mut car_reader = CarReader::new(futures::io::Cursor::new(car)).await.unwrap();
            while let Some(block) = car_reader.next_block().await.unwrap() {
                expected_store.put_keyed(&block.cid, &block.data).unwrap()
            }
            car_reader.header
        });

        assert_eq!(expected_store, actual_store);
        assert_eq!(expected_header, actual_header);
    }
}
