use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Backup your Pinboard archives
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Save your Pinboard metadata
    Metadata(MetadataFlags),
    /// Archive your cached Pinboard archives
    Archive(ArchiveFlags)
}

#[derive(Parser, Debug)]
pub struct MetadataFlags {
    #[arg(short, long)]
    /// Pinboard username.
    pub username: String,
    #[arg(short, long)]
    /// Pinboard password.
    pub password: String,
    #[arg(short, long, default_value = "pinboard.json")]
    /// File to write your Pinboard metadata to.
    pub output: String
}

#[derive(Parser, Debug)]
pub struct ArchiveFlags {
    #[arg(short, long)]
    /// Pinboard username.
    pub username: String,
    #[arg(short, long)]
    /// Pinboard password.
    pub password: String,
    #[arg(short, long, default_value = "pinboard")]
    /// Directory to save your cached Pinboard archive data to.
    pub output_directory: String,
    #[arg(short, long, default_value = "32")]
    /// Number of concurrent downloads to run at a time.
    pub concurrency: usize,
}