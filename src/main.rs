//! Lensight - A tool for adding camera information to photos
//!
//! This program processes JPEG images by adding an information bar at the bottom
//! containing camera details and EXIF information. It can process single files
//! or entire directories.

mod cli;
mod exif;
mod image_processor;
mod logo;
mod resource;
mod util;

use crate::cli::Cli;
use crate::util::{process_directory, process_single_file};
use clap::Parser;
use std::path::Path;

/// Main entry point of the application
///
/// # Returns
/// - `Result<(), Box<dyn std::error::Error>>`: Ok if successful, Err if any error occurs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let input_path = Path::new(&args.input);
    let output_path = Path::new(&args.output);

    // 检查输入路径是否存在
    if !input_path.exists() {
        println!(
            "[ERROR] Input path does not exist: {}",
            input_path.display()
        );
        return Ok(());
    }

    // 检查输出目录是否存在，如果不存在则创建
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            println!("[INFO] Creating output directory: {}", parent.display());
            std::fs::create_dir_all(parent)?;
        }
    }

    if input_path.is_dir() {
        process_directory(
            input_path,
            output_path,
            args.info_height,
            args.force_16_9,
            args.logo.as_deref(),
        )?;
    } else {
        process_single_file(
            input_path,
            output_path,
            args.info_height,
            args.force_16_9,
            args.logo.as_deref(),
        )?;
    }

    Ok(())
}
