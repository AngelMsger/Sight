//! Resource management module
//!
//! This module handles loading and managing resources such as fonts and camera logos.

use crate::logo::{logos, CameraLogos};
use rusttype::{Font, Scale};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Resources needed for image processing
#[derive(Debug)]
pub struct Resources {
    /// Bold font for camera model
    pub font_bold: Font<'static>,
    /// Regular font for lens model and settings
    pub font_regular: Font<'static>,
    /// Scale for bold font
    pub scale_bold: Scale,
    /// Scale for regular font
    pub scale_regular: Scale,
}

impl Resources {
    /// Creates a new Resources instance
    ///
    /// # Arguments
    /// * `info_height` - Height of the information bar in pixels
    ///
    /// # Returns
    /// * `Result<Resources, Box<dyn std::error::Error>>` - Ok if successful
    ///
    /// # Errors
    /// Returns an error if fonts cannot be loaded
    pub fn new(info_height: u32) -> Result<Self, Box<dyn Error>> {
        let font_bold = Self::load_font_from_file("./fonts/DejaVuSans-Bold.ttf")
            .unwrap_or_else(|_| Self::load_default_font());
        let font_regular = Self::load_font_from_file("./fonts/DejaVuSans.ttf")
            .unwrap_or_else(|_| Self::load_default_font());

        let scale_bold = Scale {
            x: info_height as f32 * 0.4,
            y: info_height as f32 * 0.4,
        };
        let scale_regular = Scale {
            x: info_height as f32 * 0.3,
            y: info_height as f32 * 0.3,
        };

        Ok(Resources {
            font_bold,
            font_regular,
            scale_bold,
            scale_regular,
        })
    }

    /// Loads a font from a file
    ///
    /// # Arguments
    /// * `path` - Path to the font file
    ///
    /// # Returns
    /// * `Result<Font<'static>, Box<dyn Error>>` - Ok if successful
    ///
    /// # Errors
    /// Returns an error if the font file cannot be read or parsed
    fn load_font_from_file(path: &str) -> Result<Font<'static>, Box<dyn Error>> {
        if !Path::new(path).exists() {
            println!("[INFO] Font file not found in {}, using default font", path);
            return Ok(Self::load_default_font());
        }

        let font_file = File::open(path)?;
        let mut font_reader = BufReader::new(&font_file);
        let mut font_data = Vec::new();
        font_reader.read_to_end(&mut font_data)?;
        let font = Font::try_from_vec(font_data)
            .ok_or_else(|| Box::<dyn Error>::from("Failed to parse font data"))?;
        Ok(font)
    }

    /// Creates a default font
    ///
    /// # Returns
    /// * `Font<'static>` - Default font
    fn load_default_font() -> Font<'static> {
        Font::try_from_vec(include_bytes!("../fonts/DejaVuSans.ttf").to_vec())
            .expect("Failed to load default font")
    }
}

/// Infers the camera brand name from a camera model string
///
/// # Arguments
/// * `camera_model` - Camera model name
///
/// # Returns
/// * `Option<String>` - Brand name if successfully inferred, None otherwise
pub fn infer_camera_brand(camera_model: &str) -> Option<String> {
    let brand = camera_model
        .to_lowercase()
        .split_whitespace()
        .next()
        .map(|s| s.to_string());

    if let Some(brand) = brand {
        if !brand.is_empty() {
            return Some(brand);
        }
    }
    None
}

/// Loads a camera logo
///
/// # Arguments
/// * `camera_model` - Camera model name
/// * `custom_logo_path` - Optional path to a custom logo file
///
/// # Returns
/// * `Result<Option<image::DynamicImage>, Box<dyn Error>>` - Ok if successful
///
/// # Errors
/// Returns an error if the logo file cannot be read or parsed
pub fn load_camera_logo(
    camera_model: &str,
    custom_logo_path: Option<&Path>,
) -> Result<Option<image::DynamicImage>, Box<dyn Error>> {
    // First try to load from custom logo file if provided
    if let Some(logo_path) = custom_logo_path {
        if !logo_path.exists() {
            println!(
                "[WARN] Custom logo file not found in {}, skipping custom logo",
                logo_path.display()
            );
        } else {
            match image::open(logo_path) {
                Ok(img) => {
                    println!("[INFO] Using custom logo file: {}", logo_path.display());
                    return Ok(Some(img));
                }
                Err(e) => {
                    println!(
                        "[WARN] Failed to load custom logo from {}: {}",
                        logo_path.display(),
                        e
                    );
                }
            }
        }
    }

    let brand = match infer_camera_brand(camera_model) {
        Some(brand) => brand,
        None => {
            println!(
                "[WARN] Could not extract brand name from camera model: {}",
                camera_model
            );
            return Ok(None);
        }
    };

    // Then try to load from external file
    let logo_path = format!("./logos/{}.png", brand);
    if Path::new(&logo_path).exists() {
        match image::open(&logo_path) {
            Ok(img) => {
                println!(
                    "[INFO] Using external logo file for camera brand '{}'",
                    brand
                );
                return Ok(Some(img));
            }
            Err(e) => {
                println!(
                    "[WARN] Failed to load logo for camera brand '{}' from file: {}",
                    brand, e
                );
            }
        }
    }

    // If external file not found or failed to load, try hardcoded base64 logo
    let base64_logo = match brand.to_lowercase().as_str() {
        "canon" => Some(logos::CANON),
        "fujifilm" => Some(logos::FUJIFILM),
        "nikon" => Some(logos::NIKON),
        "panasonic" => Some(logos::PANASONIC),
        "sony" => Some(logos::SONY),
        _ => None,
    };

    if let Some(base64_str) = base64_logo {
        match CameraLogos::load_from_base64(base64_str) {
            Ok(img) => {
                println!("[INFO] Using hardcoded logo for camera brand '{}'", brand);
                return Ok(Some(img));
            }
            Err(e) => {
                println!(
                    "[WARN] Failed to load hardcoded logo for camera brand '{}': {}",
                    brand, e
                );
            }
        }
    }

    println!(
        "[INFO] No logo available for camera brand '{}', skipping logo",
        brand
    );
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_camera_brand() {
        // Test common camera brands
        assert_eq!(
            infer_camera_brand("Canon EOS R10"),
            Some("canon".to_string())
        );
        assert_eq!(infer_camera_brand("NIKON D850"), Some("nikon".to_string()));
        assert_eq!(infer_camera_brand("SONY A7R IV"), Some("sony".to_string()));
        assert_eq!(
            infer_camera_brand("Fujifilm X-T4"),
            Some("fujifilm".to_string())
        );
        assert_eq!(
            infer_camera_brand("Panasonic Lumix S5"),
            Some("panasonic".to_string())
        );

        // Test edge cases
        assert_eq!(infer_camera_brand(""), None);
        assert_eq!(infer_camera_brand("   "), None);
        assert_eq!(infer_camera_brand("Canon"), Some("canon".to_string()));

        // Test special characters
        assert_eq!(
            infer_camera_brand("Canon-EOS-R10"),
            Some("canon-eos-r10".to_string())
        );
        assert_eq!(
            infer_camera_brand("Canon_EOS_R10"),
            Some("canon_eos_r10".to_string())
        );
    }

    #[test]
    fn test_resources_scale_calculation() {
        let info_height = 180;
        let resources = Resources::new(info_height).unwrap();

        // Test scale calculations
        assert_eq!(resources.scale_bold.x, info_height as f32 * 0.4);
        assert_eq!(resources.scale_bold.y, info_height as f32 * 0.4);
        assert_eq!(resources.scale_regular.x, info_height as f32 * 0.3);
        assert_eq!(resources.scale_regular.y, info_height as f32 * 0.3);
    }
}
