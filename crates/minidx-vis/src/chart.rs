use minidx_core::misc::ExpAvg;
use raqote::DrawTarget;

use plotters::backend::{BGRXPixel, BitMapBackend};
use plotters::prelude::*;

pub(crate) struct LineChart {
    min: (f32, f32),
    max: (f32, f32),
    data: Vec<(f32, f32)>,

    log: bool,
    smoothing_alpha: Option<f32>,
}

impl LineChart {
    pub fn new(log: bool, smoothing_alpha: Option<f32>) -> Self {
        Self {
            min: (f32::INFINITY, f32::INFINITY),
            max: (f32::NEG_INFINITY, f32::NEG_INFINITY),
            data: Vec::with_capacity(256),
            log,
            smoothing_alpha,
        }
    }

    pub fn push(&mut self, x: f32, y: f32) {
        self.min = (self.min.0.min(x), self.min.1.min(y));
        self.max = (self.max.0.max(x), self.max.1.max(y));
        self.data.push((x, y));
    }

    pub fn draw(
        &self,
        dt: &mut DrawTarget,
        margin_left: u32,
        margin_right: u32,
        margin_top: u32,
        margin_bottom: u32,
    ) {
        let size = (dt.width() as u32, dt.height() as u32);
        let mut chart_area =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(dt.get_data_u8_mut(), size)
                .unwrap()
                .into_drawing_area();

        if self.log {
            let mut chart = ChartBuilder::on(&chart_area)
                .margin_left(margin_left)
                .margin_right(margin_right)
                .margin_top(margin_top)
                .margin_bottom(margin_bottom)
                .build_cartesian_2d(self.min.0..self.max.0, (self.min.1..self.max.1).log_scale())
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(self.series(), &RED))
                .unwrap();
        } else {
            let mut chart = ChartBuilder::on(&chart_area)
                .margin_left(margin_left)
                .margin_right(margin_right)
                .margin_top(margin_top)
                .margin_bottom(margin_bottom)
                .build_cartesian_2d(self.min.0..self.max.0, self.min.1..self.max.1)
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(self.series(), &RED))
                .unwrap();
        };
    }

    fn series(&self) -> impl Iterator<Item = (f32, f32)> + use<'_> {
        let mut ea = ExpAvg::new(self.smoothing_alpha.unwrap_or(1.));
        self.data.iter().map(move |(x, y)| {
            ea.update(*y);
            (*x, ea.get().unwrap())
        })
    }
}
