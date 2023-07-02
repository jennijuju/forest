use std::{
    hint::black_box,
    io::{
        self,
        ErrorKind::{InvalidData, UnexpectedEof},
    },
};

use bytes::{Buf as _, BytesMut};
use cid::Cid;
use futures::{Stream, StreamExt as _, TryStreamExt as _};
use fvm_ipld_car::{CarHeader, CarReader};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub async fn using_car_reader(
    car: impl futures::AsyncRead + Send + Unpin,
) -> Result<
    (
        CarHeader,
        impl Stream<Item = Result<(Cid, Vec<u8>), fvm_ipld_car::Error>>,
    ),
    fvm_ipld_car::Error,
> {
    let mut car_reader = CarReader::new_unchecked(car).await?;
    let car_header = std::mem::take(&mut car_reader.header);
    Ok((
        car_header,
        futures::stream::unfold(car_reader, |mut reader| async move {
            reader.next_block().await.transpose().map(|result| {
                (
                    result.map(|fvm_ipld_car::Block { cid, data }| (cid, data)),
                    reader,
                )
            })
        }),
    ))
}

pub async fn using_tokio_codec(
    car: impl tokio::io::AsyncRead + Unpin,
) -> io::Result<(CarHeader, impl Stream<Item = io::Result<(Cid, BytesMut)>>)> {
    let mut frame_stream = tokio_util_06::codec::FramedRead::new(
        car,
        unsigned_varint::codec::UviBytes::<BytesMut>::default(),
    );
    let first_frame = frame_stream
        .next()
        .await
        .ok_or(io::Error::new(UnexpectedEof, "CAR needs a header"))??;
    let car_header =
        fvm_ipld_encoding::from_slice(&first_frame).map_err(|e| io::Error::new(InvalidData, e))?;
    Ok((
        car_header,
        frame_stream.and_then(|bytes| async move {
            let mut reader = bytes.reader();
            let cid = Cid::read_bytes(&mut reader).map_err(|e| io::Error::new(InvalidData, e))?;
            Ok((cid, reader.into_inner()))
        }),
    ))
}

fn loading_car(c: &mut Criterion) {
    let mut group = c.benchmark_group("loading_car");

    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let car = std::env::var("FOREST_BENCHMARK_SNAPSHOT").unwrap();
    let car = car.as_str();

    let first_count = tokio_runtime.block_on(async {
        using_tokio_codec(tokio::fs::File::open(car).await.unwrap())
            .await
            .unwrap()
            .1
            .try_fold(0, |count, (_cid, body)| async move {
                black_box(body);
                Ok(count + 1)
            })
            .await
            .unwrap()
    });

    group.bench_with_input(BenchmarkId::new("using_car_reader", car), car, |b, car| {
        b.to_async(criterion::async_executor::FuturesExecutor)
            .iter(|| async {
                let count = using_car_reader(async_fs::File::open(car).await.unwrap())
                    .await
                    .unwrap()
                    .1
                    .try_fold(0, |count, (_cid, body)| async move {
                        black_box(body);
                        Ok(count + 1)
                    })
                    .await
                    .unwrap();
                assert_eq!(count, first_count)
            })
    });

    group.bench_with_input(BenchmarkId::new("using_tokio_codec", car), car, |b, car| {
        b.to_async(&tokio_runtime).iter(|| async {
            let count = using_tokio_codec(tokio::fs::File::open(car).await.unwrap())
                .await
                .unwrap()
                .1
                .try_fold(0, |count, (_cid, _body)| async move { Ok(count + 1) })
                .await
                .unwrap();
            assert_eq!(count, first_count)
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = loading_car
);
criterion_main!(benches);
