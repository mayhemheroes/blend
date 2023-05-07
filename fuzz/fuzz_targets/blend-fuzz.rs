#![no_main]

use libfuzzer_sys::fuzz_target;
use blend::*;

fuzz_target!(|data: &[u8]| {
    match Blend::new(data) {
        Ok(blend) => {
            for o in blend.root_instances() {}
        },
        Err(_) => ()
    }

});
