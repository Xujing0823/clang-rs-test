use std::time::Duration;

pub trait DurationExt {
        fn to_millis(&self) -> u64;
    }

const NANOS_PER_MILLI: u32 = 1_000_000;
const MILLIS_PER_SEC: u64 = 1_000;

impl DurationExt for Duration {
        fn to_millis(&self) -> u64 {
                self.as_secs() * MILLIS_PER_SEC + (self.subsec_nanos() / NANOS_PER_MILLI) as u64
                }
    }