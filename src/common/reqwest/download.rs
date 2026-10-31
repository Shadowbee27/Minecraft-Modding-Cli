//
// Taken form https://gist.github.com/Tapanhaz/096e299bf060607b572d700e89a62529 and modified
//

use crate::debug;
use color_eyre::Result;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fmt;
use std::fs::File;
use std::io::Write;

pub struct DownloadError {}

pub async fn download_file<
  T: fmt::Display + std::fmt::Debug + reqwest::IntoUrl + std::marker::Copy,
  G: fmt::Display + std::convert::AsRef<std::path::Path> + std::marker::Copy,
>(
  client: &Client,
  url: T,
  path: G,
) -> Result<()> {
  debug!(url);

  // Reqwest setup
  let res = client.get(url).send().await?;
  let total_size = res.content_length();

  if total_size.is_some() {
    #[allow(clippy::unnecessary_unwrap)]
    let total_size = total_size.unwrap();
    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar().progress_chars("#>-")
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap());
    pb.set_message(format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
      let chunk = item?;
      file.write_all(&chunk)?;
      let new = min(downloaded + (chunk.len() as u64), total_size);
      downloaded = new;
      pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    Ok(())
  } else {
    let mut file = File::create(path)?;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
      let chunk = item?;
      file.write_all(&chunk)?;
    }
    Ok(())
  }
}

// #[cfg(test)]
// mod tests {
//   use tokio::runtime::Runtime;

//   use super::*;

//   #[test]
//   fn download_test() {
//     let rt = Runtime::new().unwrap();
//     let client = Client::new();
//     let _ = rt
//       .block_on(download_file(
//         &client,
//         "https://mirror.moson.org/arch/core/os/x86_64/linux-7.1.5.arch1-2-x86_64.pkg.tar.zst",
//         "linux.pkg.tar.zst",
//       ))
//       .unwrap();
//     let _ = rt
//       .block_on(download_file(
//         &client,
//         "https://www.reddit.com/r/rust/comments/18lrv2q/media_i_had_a_custom_ferris_plush_be_made_and_it/?tl=de#lightbox",
//         "ferris.png",
//       ))
//       .unwrap();
//     std::fs::remove_file("linux.pkg.tar.zst")
//       .expect("Failed to delete test file, manual intervention required");
//   }
// }
