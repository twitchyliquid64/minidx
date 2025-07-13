use fontdue::layout::LayoutSettings;
use minidx_core::Dtype;
use raqote::*;

mod chart;
use chart::LineChart;

mod network_traits;
pub use network_traits::VisualizableNetwork;

mod font;
pub use font::VisFont;

pub mod anim;

pub mod prelude {
    pub use crate::anim;
    pub use crate::ParamVisOpts;
    pub use crate::VisualizableNetwork;
}

/// Describes the sizing of the cell for a single parameter.
#[derive(Debug, Clone)]
pub struct ParamBox {
    w: f32,
    h: f32,
    font_size: f32,
}

impl Default for ParamBox {
    fn default() -> Self {
        Self {
            w: 40.0,
            h: 40.0,
            font_size: 17.0,
        }
    }
}

/// How to scale the representation of parameters relative to each other.
#[derive(Debug, Clone, Default)]
pub enum ParamScale {
    #[default]
    None,
    StdDev {
        mul: f32,
    },
}

// impl Default for ParamScale {
//     fn default() -> Self {
//         Self::StdDev { mul: 1.2 }
//     }
// }

/// Options for rendering a set of parameters.
#[derive(Debug, Clone)]
pub struct ParamVisOpts {
    offset: (f32, f32),
    module_padding: (f32, f32),
    cell: ParamBox,
    font: VisFont,
}

impl Default for ParamVisOpts {
    fn default() -> Self {
        let font = VisFont::default_font().unwrap();

        Self {
            offset: (2.0, 2.0),
            module_padding: (2.0, 6.0),
            cell: Default::default(),
            font,
        }
    }
}

impl ParamVisOpts {
    /// Returns a new [ParamVisOpts] with the offset updated for laying out
    /// the next module.
    pub fn update_cursor(&mut self, offset: (f32, f32)) -> &mut Self {
        self.offset.0 += offset.0;
        self.offset.1 += offset.1 + self.module_padding.1;

        self
    }

    /// A small-cell variant.
    pub fn small() -> Self {
        Self {
            cell: ParamBox {
                w: 20.0,
                h: 20.0,
                font_size: 9.0,
            },
            ..Default::default()
        }
    }
}

/// Implements visual rendering of a set of parameters.
trait PaintParams<P> {
    type Concrete: Sized;

    fn paint_params(&mut self, params: &P, opts: &mut ParamVisOpts);
    fn layout_bounds(&self, opts: &ParamVisOpts) -> (f32, f32);
}

impl PaintParams<()> for DrawTarget {
    type Concrete = ();
    fn layout_bounds(&self, opts: &ParamVisOpts) -> (f32, f32) {
        (opts.module_padding.0, opts.module_padding.1)
    }

    fn paint_params(&mut self, _params: &(), _opts: &mut ParamVisOpts) {}
}

impl<E: Dtype, const I: usize, const O: usize> PaintParams<[[E; I]; O]> for DrawTarget {
    type Concrete = [[E; I]; O];
    fn layout_bounds(&self, opts: &ParamVisOpts) -> (f32, f32) {
        (opts.cell.w * I as f32, opts.cell.h * O as f32)
    }

    fn paint_params(&mut self, params: &[[E; I]; O], opts: &mut ParamVisOpts) {
        let scale = 1.0;

        for (j, params) in params.iter().enumerate() {
            for (i, v) in params.iter().enumerate() {
                let tl = (
                    opts.offset.0 + opts.cell.w * i as f32,
                    opts.offset.1 + opts.cell.h * j as f32,
                );

                // Make box
                let mut pb = PathBuilder::new();
                pb.move_to(tl.0, tl.1);
                pb.line_to(tl.0 + opts.cell.w, tl.1);
                pb.line_to(tl.0 + opts.cell.w, tl.1 + opts.cell.h);
                pb.line_to(tl.0, tl.1 + opts.cell.h);
                pb.line_to(tl.0, tl.1);
                let p = pb.finish();

                // Paint red => grey => green background based on parameter
                let v = scale * v.to_f32().unwrap();
                self.fill(
                    &p,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(
                        0xFF,
                        ((-v).tanh().max(0.0) * 130.0) as u8 + 48,
                        (v.tanh().max(0.0) * 120.0) as u8 + 48,
                        48,
                    )),
                    &DrawOptions::new(),
                );
                // Paint grid cell boundary
                self.stroke(
                    &p,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(0xFF, 0, 0, 0)),
                    &StrokeStyle {
                        width: 1.0,
                        ..StrokeStyle::default()
                    },
                    &DrawOptions::new(),
                );

                // Generate the text
                let v_abs = v.abs();
                let mut s = if v_abs >= 10.0 {
                    format!("{:.0}", v_abs)
                } else {
                    format!("{:.1}", v_abs)
                };
                s.truncate(3);

                opts.font.raster(
                    &LayoutSettings {
                        x: tl.0,
                        y: tl.1 + 1.0,
                        max_width: Some(opts.cell.w),
                        max_height: Some(opts.cell.h - 2.0),
                        horizontal_align: fontdue::layout::HorizontalAlign::Center,
                        vertical_align: fontdue::layout::VerticalAlign::Middle,
                        ..LayoutSettings::default()
                    },
                    s.as_str(),
                    opts.cell.font_size,
                    (201, 201, 201),
                    self,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paint_params() {
        let params = &[
            [
                -10.0, -1.0, -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1,
            ],
            [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 10.0],
        ];
        let mut dt = DrawTarget::new(460, 200);
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xcf, 0xcf, 0xcf,
        ));

        dt.paint_params(params, &mut ParamVisOpts::small());

        // dt.write_png("/tmp/ye.png").expect("write failed");
    }

    #[test]
    fn test_visualize() {
        use minidx_core::layers as l;
        let network = (
            (
                l::Dense::<f32, 2, 3>::default(),
                l::Bias1d::<f32, 3>::default(),
            ),
            l::Activation::<f32>::default(),
            l::Dense::<f32, 3, 1>::default(),
        );
        let mut dt = DrawTarget::new(460, 500);
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xcf, 0xcf, 0xcf,
        ));

        let params = ParamVisOpts::default();

        use VisualizableNetwork;
        network.visualize(&mut dt, &mut params.clone());
        // dt.write_png("/tmp/ye.png").expect("write failed");
    }
}
