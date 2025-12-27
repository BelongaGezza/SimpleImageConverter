#![no_main]
use libfuzzer_sys::fuzz_target;
use mesh_core::formats::StlFormat;
use mesh_core::formats::traits::MeshReader;

fuzz_target!(|data: &[u8]| {
    // Fuzz the STL reader with arbitrary input
    let format = StlFormat::new();
    let _ = format.read(data);
    // We don't care about the result, just that it doesn't panic
});

