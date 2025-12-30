use std::time::Instant;

pub struct Timer {
    last_frame: f32,
    counter: Instant,
    pub elapsed: f32,
    pub delta: f32,
}

impl Timer {
    /// call before while loop to initialize clock
    pub fn new() -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
            last_frame: 0.0,
            counter: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.elapsed = self.counter.elapsed().as_secs_f32();
        self.delta = self.elapsed - self.last_frame;
        self.last_frame = self.elapsed;
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.delta
    }
}
