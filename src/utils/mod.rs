pub mod input;

use std::{collections::HashMap, time::{Duration, SystemTime, UNIX_EPOCH}};

pub struct TimerUtil {
    timers: HashMap<&'static str, Duration>
}

impl TimerUtil {
    pub fn new() -> Self {
        Self {
            timers: HashMap::new()
        }
    }
    pub fn start(&mut self, name: &'static str) {
        self.timers.insert(name, SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
    }
    pub fn stop(&mut self, name: &'static str) -> Duration {
        let start = self.timers.get(name).unwrap().clone(); 
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        self.timers.remove(name);

        now - start
    }
    pub fn stop_log_secs(&mut self, name: &'static str) {
        println!("{}: {:?}", name, self.stop(name))
    }
}
