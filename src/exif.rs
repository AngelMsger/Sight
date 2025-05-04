//! EXIF information processing module
//!
//! This module handles reading and processing EXIF metadata from image files.

use exif::{In, Reader, Tag};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Structure containing camera and image metadata
#[derive(Debug)]
pub struct ExifInfo {
    /// Camera model name
    pub camera_model: String,
    /// Lens model name
    pub lens_model: String,
    /// Focal length of the lens in millimeters
    pub focal_length: String,
    /// Aperture value (f-number)
    pub aperture: String,
    /// Shutter speed in seconds
    pub shutter_speed: String,
    /// ISO sensitivity value
    pub iso: String,
}

/// Reads EXIF information from an image file
///
/// # Arguments
/// * `file_path` - Path to the image file
///
/// # Returns
/// * `Result<ExifInfo, Box<dyn std::error::Error>>` - EXIF information if successful
///
/// # Errors
/// Returns an error if the file cannot be opened or if EXIF data cannot be read
pub fn read_exif_info(file_path: &Path) -> Result<ExifInfo, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(&file);
    let exif_reader = Reader::new();
    let exif = exif_reader.read_from_container(&mut buf_reader)?;

    let get_field = |tag: Tag| -> String {
        exif.get_field(tag, In::PRIMARY)
            .map(|field| field.display_value().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    Ok(ExifInfo {
        camera_model: get_field(Tag::Model),
        lens_model: get_field(Tag::LensModel),
        focal_length: get_field(Tag::FocalLength),
        aperture: get_field(Tag::FNumber),
        shutter_speed: get_field(Tag::ExposureTime),
        iso: get_field(Tag::PhotographicSensitivity),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exif_info_default_values() {
        let exif = ExifInfo {
            camera_model: "Unknown".to_string(),
            lens_model: "Unknown".to_string(),
            focal_length: "Unknown".to_string(),
            aperture: "Unknown".to_string(),
            shutter_speed: "Unknown".to_string(),
            iso: "Unknown".to_string(),
        };

        assert_eq!(exif.camera_model, "Unknown");
        assert_eq!(exif.lens_model, "Unknown");
        assert_eq!(exif.focal_length, "Unknown");
        assert_eq!(exif.aperture, "Unknown");
        assert_eq!(exif.shutter_speed, "Unknown");
        assert_eq!(exif.iso, "Unknown");
    }
}
