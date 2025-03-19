use minidx_core::optimizers::{TrainInfo, TrainParams};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::time::Instant;

/// How often something should happen, in terms of steps or time.
#[derive(Clone, Debug)]
pub enum Every {
    /// Perform the action after the specified number of steps.
    Steps(usize),
    /// Perform the action after the specified number of seconds.
    Seconds(u64),
}

impl Every {
    /// True when the action should occur.
    fn fires(&self, steps: usize, now: Instant, last_steps: usize, last_time: Instant) -> bool {
        match self {
            Every::Steps(s) => steps >= last_steps + s,
            Every::Seconds(s) => now.duration_since(last_time).as_secs() > *s,
        }
    }
}

/// Builds a recorder, which can be used to track the progress of training.
#[derive(Clone, Debug)]
pub struct RecorderBuilder {
    to_path: Option<String>,
    batch: Option<Every>,
    info: Option<Every>,
    snapshot: Option<Every>,
}

impl Default for RecorderBuilder {
    fn default() -> Self {
        Self {
            to_path: None,
            batch: Some(Every::Steps(250)),
            info: Some(Every::Steps(2000)),
            snapshot: Some(Every::Seconds(2 * 60)),
        }
    }
}

impl RecorderBuilder {
    /// Finializes the recorder, producing a recorder ready for training.
    pub fn build(self) -> std::io::Result<Recorder> {
        let now = Instant::now();

        Ok(Recorder {
            file: match self.to_path.map(|p| File::create(p)) {
                Some(r) => Some(r?),
                None => None,
            },
            sent_params: false,
            batch: self.batch.map(|e| (e, 0, now)),
            info: self.info.map(|e| (e, 0, now)),
            snapshot: self.snapshot.map(|e| (e, 0, now)),
        })
    }

    /// Configures where the recording should be saved.
    pub fn save_to<S: Into<String>>(mut self, p: S) -> Self {
        self.to_path = Some(p.into());
        self
    }

    /// Configures how often the training parameters should be recorded.
    pub fn training_params_freq(mut self, e: Every) -> Self {
        self.info = Some(e);
        self
    }

    /// Configures how often the batch statistics should be recorded.
    pub fn batch_freq(mut self, e: Every) -> Self {
        self.batch = Some(e);
        self
    }

    /// Configures how often a snapshot should be saved.
    pub fn snapshot_freq(mut self, e: Every) -> Self {
        self.snapshot = Some(e);
        self
    }
}

/// Tracks the training of a neural network, making snapshots and computing metrics.
#[derive(Debug)]
pub struct Recorder {
    file: Option<File>,

    sent_params: bool,

    // (Every, last_fired_step, last_fired_time)
    batch: Option<(Every, usize, Instant)>,
    info: Option<(Every, usize, Instant)>,
    snapshot: Option<(Every, usize, Instant)>,
}

impl Recorder {
    /// Returns a builder which can be used to construct a new [Recorder].
    pub fn new() -> RecorderBuilder {
        RecorderBuilder::default()
    }

    /// Safely flushes the recorder.
    pub fn flush(self) -> std::io::Result<()> {
        if let Some(file) = self.file {
            file.sync_all()
        } else {
            Ok(())
        }
    }

    fn write(&mut self, record: Record) -> std::io::Result<()> {
        if let Some(f) = &self.file {
            serde_json::to_writer(f, &record)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))
        } else {
            Ok(())
        }
    }

    /// Offers information about a batch training step to the recorder.
    pub fn record_batch<N: minidx_core::LoadableModule>(
        &mut self,
        info: BatchInfo,
        params: &TrainParams,
        nn: &N,
    ) -> std::io::Result<()> {
        let now = Instant::now();

        if !self.sent_params {
            self.write(Record::Params(params.clone()))?;
            self.sent_params = true;
        }

        let write_info = self
            .info
            .as_mut()
            .map(|(e, last_steps, last_time)| {
                if e.fires(info.step, now, *last_steps, *last_time) {
                    *last_steps = info.step;
                    *last_time = now;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);
        let write_batch = self
            .batch
            .as_mut()
            .map(|(e, last_steps, last_time)| {
                if e.fires(info.step, now, *last_steps, *last_time) {
                    *last_steps = info.step;
                    *last_time = now;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);
        let write_snapshot = self
            .snapshot
            .as_mut()
            .map(|(e, last_steps, last_time)| {
                if e.fires(info.step, now, *last_steps, *last_time) {
                    *last_steps = info.step;
                    *last_time = now;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if write_info {
            self.write(Record::TrainInfo(params.into()))?;
        }
        if write_batch {
            self.write(Record::Batch(info.clone()))?;
        }
        if write_snapshot {
            let mut params = HashMap::new();
            nn.save("".into(), &mut params)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

            self.write(Record::Snapshot {
                step: info.step,
                params: params,
            })?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchInfo {
    pub step: usize,
    pub size: usize,
    pub loss: f64,
    pub time_us: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Record {
    Batch(BatchInfo),
    Params(TrainParams),
    TrainInfo(TrainInfo),
    Snapshot {
        step: usize,
        params: HashMap<String, Vec<f64>>,
    },
}
