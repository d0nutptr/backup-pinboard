use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Metadata(MetadataFlags),
    Archive(ArchiveFlags)
}

#[derive(Parser, Debug)]
pub struct MetadataFlags {
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub password: String,
    #[arg(short, long, default_value = "pinboard.json")]
    pub output: String
}

#[derive(Parser, Debug)]
pub struct ArchiveFlags {
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub password: String,
    #[arg(short, long, default_value = "pinboard")]
    pub output_directory: String,
    #[arg(short, long, default_value = "32")]
    pub concurrency: usize,
}