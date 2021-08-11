#![no_main]
use libfuzzer_sys::fuzz_target;
use brainfrick::BrainFuck;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut engine = BrainFuck::new(256, "", 10000);
        engine.run(&s);
    }
});
