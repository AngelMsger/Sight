//! Command line interface module for Lensight
//!
//! This module defines the command line arguments structure and parsing logic.

use clap::Parser;
use std::path::PathBuf;

/// Command line interface for the image processing tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file or directory path
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Output file or directory path
    #[arg(value_name = "OUTPUT")]
    pub output: PathBuf,

    /// Height of the information bar in pixels
    #[arg(short, long, default_value_t = 180)]
    pub info_height: u32,

    /// Force 16:9 aspect ratio for output images
    #[arg(short, long)]
    pub force_16_9: bool,

    /// Path to a custom logo file
    #[arg(long)]
    pub logo: Option<PathBuf>,
}
