mod assets;
mod metadata;
mod cli;

use std::path::Path;
use std::process::Output;
use std::time::Duration;
use crate::cli::{ArchiveFlags, Cli, MetadataFlags, SubCommand};
use clap::Parser;
use anyhow::{bail, Context, Result};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use futures::{stream, StreamExt};


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommand::Metadata(metadata) => do_metadata(metadata).await,
        SubCommand::Archive(archive) => do_archive(archive).await
    }
}

async fn do_metadata(MetadataFlags { username, password, output, }: MetadataFlags) -> Result<()> {
    let data = metadata::get_metadata(username, password).await?;
    let mut buffer = File::create(&output).await?;
    buffer
        .write_all(&data)
        .await
        .context("Failed to write backup to archive file")
}

async fn do_archive(ArchiveFlags { username, password, output_directory, concurrency, }: ArchiveFlags) -> Result<()> {
    let cache_ids = assets::get_cache_ids(&username, &password).await?;

    let _ = create_wget_login_cookie(&username, &password).await?;

    stream::iter(cache_ids.into_iter())
        .map(|cache_id| {
            let bookmark_id = cache_id.trim_start_matches("/cached/");
            let url = format!("https://pinboard.in/{}", cache_id);
            let local_output_directory = format!("{}/{}", output_directory, bookmark_id);

            async move {
                if Path::new(&local_output_directory).exists() {
                    bail!("Local cache directory already exists");
                } else {
                    get_bookmark_contents(&local_output_directory, &url).await
                }
            }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;

    Ok(())
}

async fn get_bookmark_contents(output_directory: &str, url: &str) -> Result<Output> {
    let output_future = Command::new("wget")
        .arg("--adjust-extension")
        .arg("--span-hosts")
        .arg("--no-verbose")
        .arg("--convert-links")
        .arg("--page-requisites")
        .arg("--no-directories")
        .args(&["-e", "robots=off"])
        .args(&["--load-cookies", "/tmp/pinboard-cookies.txt"])
        .args(&["--output-file", "-"])
        .args(&["--directory-prefix", output_directory])
        .arg(&url)
        .output();

    tokio::time::timeout(Duration::from_secs(60), output_future)
        .await
        .context("Archive timed out")?
        .context("Failed to download archive from Pinboard")
}

async fn create_wget_login_cookie(username: &str, password: &str) -> Result<Output> {
    Command::new("wget")
        .args(&["--save-cookies", "/tmp/pinboard-cookies.txt"])
        .arg("--keep-session-cookies")
        .arg("--delete-after")
        .args(&["--output-file", "-"])
        .args(&["--post-data", &format!("username={}&password={}", username, password)])
        .arg("https://pinboard.in/auth/")
        .output()
        .await
        .context("Failed to run wget to get login cookies")
}