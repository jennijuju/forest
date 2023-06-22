use std::{io::ErrorKind, str::FromStr, time::Duration};

use async_compression::tokio::bufread::ZstdDecoder;
use clap::Parser;
use futures::TryStreamExt as _;
use fvm_ipld_car::CarReader;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tap::{Pipe as _, Tap};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio_util::{
    compat::TokioAsyncReadCompatExt as _,
    either::Either::{Left, Right},
};
use url::Url;

#[derive(Parser)]
struct Args {
    source: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { source } = Args::parse();

    let all_progress = MultiProgress::new();
    let source_progress = all_progress.add(ProgressBar::new_spinner().with_style(spinner_style()));
    let blocks_progress = all_progress.add(
        ProgressBar::new_spinner()
            .with_style(spinner_style())
            // we process blocks too quickly for ProgressBar::tick to be appropriate, so set a background tick instead
            .tap(|it| it.enable_steady_tick(Duration::from_millis(120))),
    );

    let maybe_compressed_source = match Url::from_str(&source) {
        Ok(url) => {
            let response = reqwest::get(url).await?.error_for_status()?;

            source_progress.set_message("fetching snapshot from network");
            if let Some(len) = response.content_length() {
                source_progress.set_length(len);
                source_progress.set_style(bytes_style())
            };

            let stream = response
                .bytes_stream()
                .map_err(|reqwest_error| std::io::Error::new(ErrorKind::Other, reqwest_error))
                .pipe(tokio_util::io::StreamReader::new);

            Left(source_progress.wrap_async_read(stream))
        }
        Err(_) => {
            let stream = tokio::fs::File::open(source).await?;

            source_progress.set_message("reading snapshot from disk");
            source_progress.set_length(stream.metadata().await?.len());
            source_progress.set_style(bytes_style());

            Right(source_progress.wrap_async_read(stream))
        }
    };

    let source = {
        // The underlying stream may or may not be compressed
        // Check for a zstd header and uncompress if required

        // https://github.com/facebook/zstd/blob/e4aeaebc201ba49fec50b087aeb15343c63712e5/doc/zstd_compression_format.md#zstandard-frames
        // buffer should be >= 18 bytes
        let buffer_size = 8_000usize.next_power_of_two();
        let mut buf_reader = BufReader::with_capacity(buffer_size, maybe_compressed_source);
        let peeked = buf_reader.fill_buf().await?;
        match zstd_safe::get_frame_content_size(peeked) {
            Ok(_compressed) => {
                all_progress.println("snapshot is compressed")?;
                // We not `consume`d anything from the bufreader, so fine to pass through
                Left(ZstdDecoder::new(buf_reader))
            }
            Err(_uncompressed) => {
                all_progress.println("snapshot is uncompressed")?;
                Right(buf_reader)
            }
        }
    };

    let mut car_reader = CarReader::new(source.compat()).await?;

    let mut count = 0;
    while let Some(_block) = car_reader.next_block().await? {
        count += 1;
        blocks_progress.set_message(format!("loaded {count} blocks"));
    }

    Ok(())
}

pub fn bytes_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{msg:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}",
    )
    .expect("invalid progress template")
    .progress_chars("=>-")
}

pub fn spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.blue} {msg}")
        .unwrap()
        .tick_strings(&[
            "▹▹▹▹▹",
            "▸▹▹▹▹",
            "▹▸▹▹▹",
            "▹▹▸▹▹",
            "▹▹▹▸▹",
            "▹▹▹▹▸",
            "▪▪▪▪▪",
        ])
}
