// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{DynamicImage, ImageBuffer, ImageFormat as ImgFormat, Rgb};
use img_core::{FormatRegistry, ImageConverter, ImageFormat, QualitySettings};

/// Helper to create a test PNG image
fn create_test_png(width: u32, height: u32) -> Vec<u8> {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        Rgb([(x % 256) as u8, (y % 256) as u8, 128])
    });
    let mut buffer = Vec::new();
    DynamicImage::ImageRgb8(img)
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImgFormat::Png)
        .unwrap();
    buffer
}

/// Helper to create a test JPEG image
fn create_test_jpeg(width: u32, height: u32) -> Vec<u8> {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        Rgb([(x % 256) as u8, (y % 256) as u8, 128])
    });
    let mut buffer = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 90);
    encoder
        .encode(img.as_raw(), width, height, image::ExtendedColorType::Rgb8)
        .unwrap();
    buffer
}

fn benchmark_png_read(c: &mut Criterion) {
    let png_data = create_test_png(100, 100);
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();

    c.bench_function("png_read_100x100", |b| {
        b.iter(|| {
            black_box(reader.read(black_box(&png_data)).unwrap());
        });
    });
}

fn benchmark_png_write(c: &mut Criterion) {
    let png_data = create_test_png(100, 100);
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();
    let quality = QualitySettings::default();
    let image = reader.read(&png_data).unwrap();

    c.bench_function("png_write_100x100", |b| {
        b.iter(|| {
            black_box(
                writer
                    .write(black_box(&image), black_box(&quality))
                    .unwrap(),
            );
        });
    });
}

fn benchmark_jpeg_read(c: &mut Criterion) {
    let jpeg_data = create_test_jpeg(100, 100);
    let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();

    c.bench_function("jpeg_read_100x100", |b| {
        b.iter(|| {
            black_box(reader.read(black_box(&jpeg_data)).unwrap());
        });
    });
}

fn benchmark_jpeg_write(c: &mut Criterion) {
    let jpeg_data = create_test_jpeg(100, 100);
    let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let quality = QualitySettings::new(90);
    let image = reader.read(&jpeg_data).unwrap();

    c.bench_function("jpeg_write_100x100", |b| {
        b.iter(|| {
            black_box(
                writer
                    .write(black_box(&image), black_box(&quality))
                    .unwrap(),
            );
        });
    });
}

fn benchmark_png_to_jpeg(c: &mut Criterion) {
    let png_data = create_test_png(100, 100);
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    c.bench_function("png_to_jpeg_100x100", |b| {
        b.iter(|| {
            black_box(
                converter
                    .convert(
                        black_box(&png_data),
                        reader.as_ref(),
                        writer.as_ref(),
                        black_box(&quality),
                    )
                    .unwrap(),
            );
        });
    });
}

fn benchmark_jpeg_to_png(c: &mut Criterion) {
    let jpeg_data = create_test_jpeg(100, 100);
    let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::default();

    c.bench_function("jpeg_to_png_100x100", |b| {
        b.iter(|| {
            black_box(
                converter
                    .convert(
                        black_box(&jpeg_data),
                        reader.as_ref(),
                        writer.as_ref(),
                        black_box(&quality),
                    )
                    .unwrap(),
            );
        });
    });
}

fn benchmark_large_image(c: &mut Criterion) {
    let png_data = create_test_png(1000, 1000);
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    c.bench_function("png_to_jpeg_1000x1000", |b| {
        b.iter(|| {
            black_box(
                converter
                    .convert(
                        black_box(&png_data),
                        reader.as_ref(),
                        writer.as_ref(),
                        black_box(&quality),
                    )
                    .unwrap(),
            );
        });
    });
}

criterion_group!(
    benches,
    benchmark_png_read,
    benchmark_png_write,
    benchmark_jpeg_read,
    benchmark_jpeg_write,
    benchmark_png_to_jpeg,
    benchmark_jpeg_to_png,
    benchmark_large_image
);
criterion_main!(benches);
