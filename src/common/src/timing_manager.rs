use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Timing {
    last_frame_time: Instant,
    delta_time: Duration,
}

pub struct TimingManager {
    timings: HashMap<String, Timing>,
}

impl TimingManager {
    pub fn new() -> Self {
        TimingManager {
            timings: HashMap::new(),
        }
    }

    pub fn start_frame(&mut self, name: &str) {
        let timing = self.timings
            .entry(name.to_string())
            .or_insert(Timing {
                last_frame_time: Instant::now(),
                delta_time: Duration::from_secs(0),
        });

        timing.last_frame_time = Instant::now();
    }

    pub fn end_frame(&mut self, name: &str) {
        let timing = self.timings
            .get_mut(name)
            .expect("No timing found for this name");

        timing.delta_time = timing.last_frame_time
            .elapsed();
    }

    pub fn get_delta_time(&self, name: &str) -> Duration {
        self.timings
            .get(name)
            .expect("No timing found for this name")
            .delta_time
    }
}