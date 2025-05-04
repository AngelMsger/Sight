# Changelog

This project follows [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2024-05-04

### Added

- Initial release
- Support for adding camera information and shooting details to photos
- Support for single image and batch processing
- Built-in support for major camera brands (Canon, Nikon, Sony, Fujifilm, Panasonic)
- Optional 16:9 aspect ratio output
- Custom logo support
- Customizable information bar height with adaptive font and logo sizing

### Technical Details

- Using Rust 2021 edition
- Key dependencies:
  - image 0.24.7
  - kamadak-exif 0.5.5
  - clap 4.4.11
  - walkdir 2.4.0
  - imageproc 0.23.0
  - rusttype 0.9.3
  - base64 0.21.5

## [Unreleased]

### Todo

- Add support for more camera brands
- Performance optimization
- Error handling improvements
- Additional customization options
