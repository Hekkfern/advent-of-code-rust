pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn distance(&self) -> u64 {
        self.distance
    }
}
