use crate::{LineChart, ParamVisOpts, VisualizableNetwork};
use raqote::{DrawTarget, SolidSource};
use std::sync::mpsc::{channel, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use plotters::backend::{BGRXPixel, BitMapBackend};
use plotters::prelude::*;

use fontdue::layout::LayoutSettings;

#[derive(Debug)]
pub enum RecorderErr {
    Recv(RecvError),
    Pipe(std::io::Error),
    Run(std::io::Error),
    Write(std::io::Error),
    Flush(std::io::Error),
}

/// Records training progress to a video.
pub struct Recorder<N: VisualizableNetwork<DrawTarget> + std::marker::Send> {
    marker: std::marker::PhantomData<N>,
    sender: Option<Sender<(f32, f32, N)>>,
    final_error: Arc<Mutex<Option<RecorderErr>>>,
}

impl<N: VisualizableNetwork<DrawTarget> + std::marker::Send + 'static> Recorder<N> {
    pub fn mp4(path: &str, size: (usize, usize), mut opts: ParamVisOpts) -> Self {
        let (sender, receiver) = channel();
        let err = Arc::<Mutex<Option<RecorderErr>>>::default();

        let s = Self {
            sender: Some(sender),
            marker: Default::default(),
            final_error: err.clone(),
        };

        let path = path.to_string();
        thread::spawn(move || {
            let mut err = err.lock().unwrap();
            let mut loss_chart = LineChart::new(false, Some(0.5));

            // Offset parameters render to the right of the screen
            opts.offset.0 += (size.0 / 2) as f32;

            let res = make_video(size, path.as_str(), |dt, _n| match receiver.recv() {
                Err(e) => {
                    *err = Some(RecorderErr::Recv(e));
                    return false;
                }
                Ok((epoch, loss, network)) => {
                    loss_chart.push(epoch, loss);

                    // Render the parameter visualization, right
                    dt.clear(SolidSource::from_unpremultiplied_argb(
                        0xff, 0xcf, 0xcf, 0xcf,
                    ));
                    network.visualize(dt, &mut opts.clone());

                    // Render the plot, left
                    loss_chart.draw(dt, 5, size.0 as u32 / 2, 50, 50);

                    opts.font.raster(
                        &LayoutSettings {
                            x: 1 as f32,
                            y: (size.1 - 48) as f32,
                            max_width: Some(size.0 as f32 / 2.0),
                            max_height: Some(50.0),
                            horizontal_align: fontdue::layout::HorizontalAlign::Left,
                            vertical_align: fontdue::layout::VerticalAlign::Bottom,
                            ..LayoutSettings::default()
                        },
                        format!("L: {:05.3} - N: {:05}", loss, epoch).as_str(),
                        30.0,
                        (10, 10, 10),
                        dt,
                    );

                    return true;
                }
            });

            if let Err(e) = res {
                *err = Some(e);
            }
        });

        s
    }

    /// Push sends a checkpoint and its corresponding loss value to the recorder.
    ///
    /// Each call to push corresponds with a single frame.
    pub fn push(&mut self, epoch: f32, loss: f32, network: N) {
        if let Some(sender) = &self.sender {
            sender.send((epoch, loss, network)).unwrap();
        }
    }

    /// Blocks till the render is complete and returns the error. Must only be called once.
    pub fn wait(&mut self) -> Option<RecorderErr> {
        self.sender = None;
        self.final_error.lock().unwrap().take()
    }
}

/// Renders a video file by calling the given function to generate every frame.
///
/// The callback function should write the frame in rgb (3x8 bits a pixel) format,
/// and return true if the callback should be called again to generate the next
/// frame.
///
/// The video is encoded as an MP4 at 30 fps.
#[cfg(not(target_os = "windows"))]
fn make_video<F: FnMut(&mut DrawTarget, usize) -> bool>(
    size: (usize, usize),
    out_path: &str,
    mut frame_cb: F,
) -> Result<(), RecorderErr> {
    let (w, h) = size;
    let mut dt = DrawTarget::new(w as i32, h as i32);

    let (reader, mut writer) = os_pipe::pipe().map_err(RecorderErr::Pipe)?;

    use command_fds::CommandFdExt;
    let mut command = std::process::Command::new("ffmpeg");
    command
        .args([
            "-f",
            "rawvideo",
            "-video_size",
            &format!("{}x{}", w, h),
            "-pixel_format",
            "bgra",
            "-framerate",
            "30/1",
        ])
        .arg("-i")
        .arg("pipe:3")
        .args([
            "-c:v",
            "libx264",
            "-crf",
            "21",
            "-profile:v",
            "baseline",
            "-level",
            "3.0",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "faststart",
        ])
        .arg("-y")
        .arg(out_path);

    command
        .fd_mappings(vec![command_fds::FdMapping {
            parent_fd: reader.into(),
            child_fd: 3,
        }])
        .unwrap();

    let mut child = command.spawn().map_err(RecorderErr::Run)?;

    let mut n = 0;
    while frame_cb(&mut dt, n) {
        use std::io::Write;
        writer
            .write_all(dt.get_data_u8())
            .map_err(RecorderErr::Write)?;
        n += 1;
        if n == 4 {
            writer.flush().map_err(RecorderErr::Flush)?; // get ffmpeg encoding early on
        }
    }

    drop(writer);
    child.wait().unwrap();
    Ok(())
}
