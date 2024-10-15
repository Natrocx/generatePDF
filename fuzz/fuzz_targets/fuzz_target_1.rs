#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|byte_len: usize| {
    // normalise input to prevent overflowing ram
    let min_value  = 544;
    let max_value = 1 << 28;

    // Scale the fuzzed `data` to the range 544..2^28
    let range = max_value - min_value;
    let byte_len: usize = min_value + (byte_len % range);

    generatePDF::generate_pdf_with_size(byte_len).unwrap().save("fuzz.pdf").unwrap();
    assert_eq!(std::fs::metadata("fuzz.pdf").unwrap().len() as usize, byte_len);
});
