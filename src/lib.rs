//! Lensight - A tool for adding camera information to photos
//!
//! This library provides functionality to process JPEG images by adding an information bar
//! containing camera details and EXIF information.

pub mod cli;
pub mod exif;
pub mod image_processor;
pub mod logo;
pub mod resource;
pub mod util;

pub use util::{process_directory, process_single_file};
