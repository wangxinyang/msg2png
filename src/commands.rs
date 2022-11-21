use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),

    Decode(DecodeArgs),

    Remove(RemoveArgs),

    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    // file path
    pub file_path: PathBuf,

    // chunk type
    pub chunk_type: String,

    // message
    pub message: String,

    // optional output file
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,

    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,

    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}

pub fn parse_commands() -> Result<Cli> {
    Ok(Cli::parse())
}
