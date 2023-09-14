use bytes::{BufMut as _, Bytes, BytesMut};
use cid::Cid;
use futures::{Sink, SinkExt as _};
use pin_project_lite::pin_project;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::AsyncWrite;
use tokio_util::codec::FramedWrite;
use unsigned_varint::codec::UviBytes as VarintCodec;

use crate::db::car::plain::cid_error_to_io_error;

// TODO(aatifsyed): `car_stream` should not be a submodule of `db`
use super::db::car_stream::{Block, CarHeader};

pin_project! {
pub struct CarV1<W> {
    #[pin]
    inner: FramedWrite<W, VarintCodec>,
}}

impl<W> CarV1<W>
where
    // TODO(aatifsyed): remove Unpin bound
    W: AsyncWrite + Unpin,
{
    pub async fn new(writer: W, roots: Vec<Cid>) -> anyhow::Result<Self> {
        let mut inner = FramedWrite::new(writer, VarintCodec::default());
        inner
            .send(Bytes::from(fvm_ipld_encoding::to_vec(&CarHeader {
                roots,
                version: 1,
            })?))
            .await?;
        Ok(Self { inner })
    }
}

impl<W> Sink<Block> for CarV1<W>
where
    W: AsyncWrite,
{
    type Error = io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Block) -> io::Result<()> {
        // TODO(aatifsyed): Should `Block::write` exist at all?
        let mut encoded = BytesMut::new();
        let Block { cid, data } = item;
        cid.write_bytes((&mut encoded).writer())
            .map_err(cid_error_to_io_error)?;
        encoded.extend(data);

        self.project().inner.start_send(encoded.freeze())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_close(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_compression::tokio::write::ZstdEncoder;
    use futures::{executor::block_on, stream, StreamExt as _};
    use quickcheck::quickcheck;

    quickcheck! {
        fn zstd_always_valid(blocks: Vec<Block>) -> () {
            block_on(async {
                let mut car_zst = vec![];
                stream::iter(blocks)
                    .map(io::Result::Ok)
                    .forward(
                        CarV1::new(ZstdEncoder::new(&mut car_zst), vec![])
                            .await
                            .unwrap()
                        )
                    .await
                    .unwrap();

                // check it's a valid zstd archive
                zstd::decode_all(car_zst.as_slice()).unwrap();
            })
        }
    }

    #[test]
    #[should_panic = "incomplete frame"] // https://github.com/ChainSafe/forest/issues/3485
    fn zstd_decode_all_detects_broken_archives() {
        zstd::decode_all(include_bytes!("../../assets/actor_bundles.car.zst").as_slice()).unwrap();
    }
}
