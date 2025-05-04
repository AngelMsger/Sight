<div align="center">
  <img src="docs/lensight_logo.jpg" alt="Lensight Logo" width="128" height="128" style="border-radius: 32px">
  <br>
  <a href="https://github.com/AngelMsger/lensight/actions/workflows/ci.yml">
    <img src="https://github.com/AngelMsger/lensight/actions/workflows/ci.yml/badge.svg" alt="Build Status">
  </a>
</div>

# Lensight

A command-line tool for adding camera information and shooting details to photos.

## Features

- Add camera information and shooting details to photos
- Support for single image and batch processing
- Built-in support for major camera brands (Canon, Nikon, Sony, Fujifilm, Panasonic)
- Optional 16:9 aspect ratio output for video platforms
- Custom logo support
- Customizable information bar height with adaptive font and logo sizing

## Installation

### From Source

1. Ensure you have Rust and Cargo installed
2. Clone the repository:
   ```bash
   git clone git@github.com:AngelMsger/lensight.git
   cd lensight
   ```
3. Build and install:
   ```bash
   cargo install --path .
   ```

### From crates.io

```bash
cargo install lensight
```

## Usage

### Basic Commands

#### Single Image Processing

```bash
lensight input.jpg output.jpg
```

#### Batch Processing

```bash
lensight ./input_directory ./output_directory
```

### Advanced Options

#### Force 16:9 Aspect Ratio

```bash
lensight --force-16-9 input.jpg output.jpg
```

#### Custom Information Bar Height

```bash
lensight ./input ./output --info-height 240
```

#### Custom Logo

When installed via Cargo, Lensight includes built-in logos for Canon, Nikon, Sony, Fujifilm, and Panasonic. When building from source, you can add custom logos by placing them in the `logos` directory with the brand name in lowercase.

To specify a custom logo file:

```bash
lensight ./input ./output --logo logos/custom.png
```

## Examples

Original image:
![Original](docs/IMG_0197.jpg)

Processed result:
![Processed](docs/IMG_0197_info.jpg)

16:9 aspect ratio result:
![16:9 Result](docs/IMG_0197_info_16_9.jpg)

> Note: Please do not use the sample images in the repository for testing as they have been compressed and lost their EXIF data for preview purposes.

## Related Projects

- [**camera-watermark**](https://github.com/dearDreamWeb/camera-watermark): This project uses the logo files organized by this project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
