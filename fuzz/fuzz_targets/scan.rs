#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // For any string, parsing it may or may not succeed, but it should not crash,
    // hang, or panic.
    let _ = rsj::scan::scan_sentence(data);
});
