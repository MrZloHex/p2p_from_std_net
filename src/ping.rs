use std::time::Duration;

pub struct Ping {
    config: PingConfig,
    events: Vec<PingEvent>
}

impl Ping {
    pub fn new(config: PingConfig) -> Self {
        Ping {
            config,
            events: Vec::new()
        }
    }
}

pub struct PingEvent {
    pub author: String,
    pub peers: Vec<String>
}

pub struct PingConfig {
    period: Duration,
    keep_alive: bool
}

impl PingConfig {
    pub fn new() -> Self {
        Self {
            period: Duration::from_secs(10),
            keep_alive: false
        }
    }

    pub fn set_period(mut self, period: Duration) -> Self {
        self.period = period;
        self
    }

    pub fn set_keep_alive(mut self, alive: bool) -> Self {
        self.keep_alive = alive;
        self
    }
}
