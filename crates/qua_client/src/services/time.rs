use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

pub struct TimeService;

impl TimeService {
    pub fn now() -> SystemTime {
        let performance = web_sys::window()
            .expect("should have a window in this context")
            .performance()
            .expect("performance should be available");

        let time = performance.now();

        let secs = (time as u64) / 1_000;
        let nanos = (((time as u64) % 1_000) as u32) * 1_000_000;

        UNIX_EPOCH + Duration::new(secs, nanos)
    }
}
