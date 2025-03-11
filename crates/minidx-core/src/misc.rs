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

fn cosine_decay(current_timestep: usize, end_timestep: usize, start_val: f32, end_val: f32) -> f32 {
    if current_timestep >= end_timestep {
        return end_val;
    }

    use std::f32::consts::PI;
    let decay_ratio = (current_timestep as f32) / (end_timestep as f32);
    let cosine_decay = 0.5 * (1.0 + (PI * decay_ratio).cos());
    end_val + (start_val - end_val) * cosine_decay
}

/// A value which can decay according to some schedule over the progression of timesteps.
#[derive(Clone, Debug)]
pub enum Decay {
    None(f32),
    Linear {
        start: f32,
        decay: f32,
    },
    Cosine {
        start: f32,
        end: f32,
        num_steps: usize,
    },
}

impl Decay {
    pub fn start_value(&self) -> f32 {
        use Decay::*;
        match self {
            None(v) => *v,
            Linear { start, .. } => *start,
            Cosine { start, .. } => *start,
        }
    }

    pub fn at_timestep(&self, timestep: usize) -> f32 {
        use Decay::*;
        match self {
            None(v) => *v,
            Linear { start, decay } => {
                let decayed = *start - (*decay * timestep as f32);
                if decayed < *decay {
                    *decay
                } else {
                    decayed
                }
            }
            Cosine {
                start,
                end,
                num_steps,
            } => cosine_decay(timestep, *num_steps, *start, *end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_linear() {
        let decay = Decay::Linear {
            start: 2.0,
            decay: 0.25,
        };
        assert_eq!(decay.at_timestep(0), 2.0);
        assert_eq!(decay.at_timestep(1), 1.75);
        assert_eq!(decay.at_timestep(999999), 0.25);
    }

    #[test]
    fn test_decay_cosine() {
        let decay = Decay::Cosine {
            start: 2.0,
            end: 1.0,
            num_steps: 1000,
        };
        assert_eq!(decay.at_timestep(0), 2.0);

        let mid = decay.at_timestep(500);
        assert!(mid > 1.49);
        assert!(mid < 1.51);

        assert_eq!(decay.at_timestep(999999), 1.0);
    }
}
