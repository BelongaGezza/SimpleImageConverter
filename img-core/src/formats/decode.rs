// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ColorType, ImageData};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use image::{DynamicImage, GenericImageView, ImageFormat};

pub fn read_dynamic_image(
    data: &[u8],
    format: ImageFormat,
    limits: &ResourceLimits,
) -> Result<DynamicImage> {
    limits.check_file_size(data.len())?;

    if let Some((width, height)) = preflight_dimensions(data, format)? {
        limits.check_decoded_image_size(width, height, 4)?;
    }

    let img = image::load_from_memory_with_format(data, format).map_err(|e| {
        ConversionError::ConversionFailed(format!("Failed to decode image data: {}", e))
    })?;

    let (width, height) = img.dimensions();
    limits.check_decoded_image_size(width, height, bytes_per_pixel_for_dynamic(&img))?;

    Ok(img)
}

pub fn dynamic_to_image_data(img: DynamicImage, limits: &ResourceLimits) -> Result<ImageData> {
    let (width, height) = img.dimensions();
    let color_type = match img {
        DynamicImage::ImageLuma8(_) => ColorType::Grayscale,
        DynamicImage::ImageLumaA8(_) => ColorType::GrayscaleAlpha,
        DynamicImage::ImageRgb8(_) => ColorType::Rgb,
        DynamicImage::ImageRgba8(_) => ColorType::Rgba,
        _ => {
            limits.check_decoded_image_size(width, height, 4)?;
            let rgba = img.to_rgba8();
            let image = ImageData {
                width,
                height,
                data: rgba.into_raw(),
                color_type: ColorType::Rgba,
            };
            crate::validation::validate_image_data_with_limits(&image, limits)?;
            return Ok(image);
        }
    };

    let bytes_per_pixel = match color_type {
        ColorType::Rgb => 3,
        ColorType::Rgba => 4,
        ColorType::Grayscale => 1,
        ColorType::GrayscaleAlpha => 2,
    };
    limits.check_decoded_image_size(width, height, bytes_per_pixel)?;

    let data = match img {
        DynamicImage::ImageLuma8(img) => img.into_raw(),
        DynamicImage::ImageLumaA8(img) => img.into_raw(),
        DynamicImage::ImageRgb8(img) => img.into_raw(),
        DynamicImage::ImageRgba8(img) => img.into_raw(),
        _ => unreachable!("non-standard dynamic image handled above"),
    };

    let image = ImageData {
        width,
        height,
        data,
        color_type,
    };
    crate::validation::validate_image_data_with_limits(&image, limits)?;
    Ok(image)
}

pub fn dynamic_to_rgb_image_data(img: DynamicImage, limits: &ResourceLimits) -> Result<ImageData> {
    let (width, height) = img.dimensions();
    limits.check_decoded_image_size(width, height, 3)?;
    let rgb = img.to_rgb8();
    let image = ImageData {
        width,
        height,
        data: rgb.into_raw(),
        color_type: ColorType::Rgb,
    };
    crate::validation::validate_image_data_with_limits(&image, limits)?;
    Ok(image)
}

pub fn dynamic_to_webp_image_data(img: DynamicImage, limits: &ResourceLimits) -> Result<ImageData> {
    let (width, height) = img.dimensions();
    if img.color().has_alpha() {
        limits.check_decoded_image_size(width, height, 4)?;
        let rgba = img.to_rgba8();
        let image = ImageData {
            width,
            height,
            data: rgba.into_raw(),
            color_type: ColorType::Rgba,
        };
        crate::validation::validate_image_data_with_limits(&image, limits)?;
        Ok(image)
    } else {
        dynamic_to_rgb_image_data(img, limits)
    }
}

fn bytes_per_pixel_for_dynamic(img: &DynamicImage) -> usize {
    match img {
        DynamicImage::ImageLuma8(_) => 1,
        DynamicImage::ImageLumaA8(_) => 2,
        DynamicImage::ImageRgb8(_) => 3,
        DynamicImage::ImageRgba8(_) => 4,
        _ => 4,
    }
}

fn preflight_dimensions(data: &[u8], format: ImageFormat) -> Result<Option<(u32, u32)>> {
    match format {
        ImageFormat::Png => parse_png_dimensions(data).map(Some),
        ImageFormat::Gif => parse_gif_dimensions(data).map(Some),
        ImageFormat::Bmp => parse_bmp_dimensions(data).map(Some),
        ImageFormat::Jpeg => parse_jpeg_dimensions(data).map(Some),
        ImageFormat::WebP => Ok(parse_webp_dimensions(data)),
        _ => Ok(None),
    }
}

fn parse_png_dimensions(data: &[u8]) -> Result<(u32, u32)> {
    if data.len() < 24 || &data[0..8] != b"\x89PNG\r\n\x1a\n" || &data[12..16] != b"IHDR" {
        return Err(ConversionError::InvalidFormat(
            "Invalid PNG header".to_string(),
        ));
    }
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Ok((width, height))
}

fn parse_gif_dimensions(data: &[u8]) -> Result<(u32, u32)> {
    if data.len() < 10 || !data.starts_with(b"GIF8") {
        return Err(ConversionError::InvalidFormat(
            "Invalid GIF header".to_string(),
        ));
    }
    let width = u16::from_le_bytes([data[6], data[7]]) as u32;
    let height = u16::from_le_bytes([data[8], data[9]]) as u32;
    Ok((width, height))
}

fn parse_bmp_dimensions(data: &[u8]) -> Result<(u32, u32)> {
    if data.len() < 26 || &data[0..2] != b"BM" {
        return Err(ConversionError::InvalidFormat(
            "Invalid BMP header".to_string(),
        ));
    }
    let width = i32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    if width <= 0 || height == 0 {
        return Err(ConversionError::InvalidInput(
            "Invalid BMP dimensions".to_string(),
        ));
    }
    Ok((width as u32, height.unsigned_abs()))
}

fn parse_jpeg_dimensions(data: &[u8]) -> Result<(u32, u32)> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err(ConversionError::InvalidFormat(
            "Invalid JPEG header".to_string(),
        ));
    }

    let mut i = 2usize;
    while i + 3 < data.len() {
        while i < data.len() && data[i] == 0xFF {
            i += 1;
        }
        if i >= data.len() {
            break;
        }
        let marker = data[i];
        i += 1;
        if marker == 0xD9 || marker == 0xDA {
            break;
        }
        if i + 2 > data.len() {
            break;
        }
        let len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        if len < 2 || i + len > data.len() {
            break;
        }
        if matches!(
            marker,
            0xC0 | 0xC1
                | 0xC2
                | 0xC3
                | 0xC5
                | 0xC6
                | 0xC7
                | 0xC9
                | 0xCA
                | 0xCB
                | 0xCD
                | 0xCE
                | 0xCF
        ) && len >= 7
        {
            let height = u16::from_be_bytes([data[i + 3], data[i + 4]]) as u32;
            let width = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u32;
            return Ok((width, height));
        }
        i += len;
    }

    Ok((1, 1))
}

fn parse_webp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 30 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        return None;
    }
    match &data[12..16] {
        b"VP8X" if data.len() >= 30 => {
            let width = 1 + u32::from_le_bytes([data[24], data[25], data[26], 0]);
            let height = 1 + u32::from_le_bytes([data[27], data[28], data[29], 0]);
            Some((width, height))
        }
        _ => None,
    }
}
