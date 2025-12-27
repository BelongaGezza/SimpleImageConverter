#![no_main]
use libfuzzer_sys::fuzz_target;
use img_core::formats::JpegFormat;
use img_core::formats::traits::ImageReader;

fuzz_target!(|data: &[u8]| {
    // Fuzz the JPEG reader with arbitrary input
    let format = JpegFormat::new();
    let _ = format.read(data);
    // We don't care about the result, just that it doesn't panic
});

