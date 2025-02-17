pub struct ExpAvg {
    alpha: f32,       // Smoothing factor (0 < alpha ≤ 1)
    avg: Option<f32>, // Optional to handle the first sample
}

impl ExpAvg {
    /// Creates a new `ExpAvg` with the given smoothing factor.
    pub fn new(alpha: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&alpha),
            "Alpha must be in the range (0,1]"
        );
        Self { alpha, avg: None }
    }

    /// Updates the moving average with a new sample.
    pub fn update(&mut self, sample: f32) {
        self.avg = Some(match self.avg {
            Some(current_avg) => self.alpha * sample + (1.0 - self.alpha) * current_avg,
            None => sample, // Initialize with the first sample
        });
    }

    /// Returns the current average.
    pub fn get(&self) -> Option<f32> {
        self.avg
    }
}
