#![no_main]
use libfuzzer_sys::fuzz_target;
use img_core::formats::PngFormat;
use img_core::formats::traits::ImageReader;

fuzz_target!(|data: &[u8]| {
    // Fuzz the PNG reader with arbitrary input
    let format = PngFormat::new();
    let _ = format.read(data);
    // We don't care about the result, just that it doesn't panic
});

