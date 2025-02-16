use fontdue::layout::{Layout, LayoutSettings, TextStyle};
use fontdue::Font;
use minidx_core::Dtype;
use raqote::*;

mod network_traits;
pub use network_traits::VisualizableNetwork;

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
    scale: ParamScale,
    font: std::sync::Arc<Font>,
}

impl Default for ParamVisOpts {
    fn default() -> Self {
        use rust_fontconfig::{FcFontCache, FcPattern};
        let cache = FcFontCache::build();
        // println!("{:?}", cache.list());
        let results = cache
            .query(&FcPattern {
                family: Some(String::from("Liberation Mono")), // TODO: Better search logic
                ..Default::default()
            })
            .unwrap();

        let font_bytes = std::fs::read(results.path.clone()).unwrap();

        Self {
            offset: (2.0, 2.0),
            module_padding: (2.0, 6.0),
            cell: Default::default(),
            scale: Default::default(),
            font: std::sync::Arc::new(
                Font::from_bytes(
                    font_bytes,
                    fontdue::FontSettings {
                        collection_index: results.font_index as u32,
                        ..Default::default()
                    },
                )
                .unwrap(),
            ),
        }
    }
}

impl ParamVisOpts {
    /// Returns a new [ParamVisOpts] with the offset updated for laying out
    /// the next module.
    pub fn update_cursor(&self, offset: (f32, f32)) -> Self {
        Self {
            offset: (
                offset.0 + self.offset.0,
                offset.1 + self.offset.1 + self.module_padding.1,
            ),
            ..self.clone()
        }
    }
}

/// Implements visual rendering of a set of parameters.
trait PaintParams<P> {
    type Concrete: Sized;

    fn paint_params(&mut self, params: &P, opts: &ParamVisOpts);
    fn layout_bounds(&self, opts: &ParamVisOpts) -> (f32, f32);
}

impl<E: Dtype, const I: usize, const O: usize> PaintParams<[[E; I]; O]> for DrawTarget {
    type Concrete = [[E; I]; O];
    fn layout_bounds(&self, opts: &ParamVisOpts) -> (f32, f32) {
        (
            opts.offset.0 + opts.cell.w * I as f32,
            opts.offset.1 + opts.cell.h * O as f32,
        )
    }

    fn paint_params(&mut self, params: &[[E; I]; O], opts: &ParamVisOpts) {
        let mut layout = Layout::<()>::new(fontdue::layout::CoordinateSystem::PositiveYDown);

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

                // Layout the text
                layout.reset(&LayoutSettings {
                    x: tl.0,
                    y: tl.1 + 1.0,
                    max_width: Some(opts.cell.w),
                    max_height: Some(opts.cell.h - 2.0),
                    horizontal_align: fontdue::layout::HorizontalAlign::Center,
                    vertical_align: fontdue::layout::VerticalAlign::Middle,
                    ..LayoutSettings::default()
                });
                layout.append(
                    &[opts.font.clone()], // its an arc
                    &TextStyle::new(s.as_str(), opts.cell.font_size, 0),
                );

                // Raster the text
                let (rc, gc, bc) = (201, 201, 201);
                for g in layout.glyphs().iter() {
                    let (_, b) = opts.font.rasterize_config(g.key);

                    let mut buf = Vec::new();
                    buf.resize(g.width * g.height, 0);
                    for (i, x) in b.into_iter().enumerate() {
                        let s = SolidSource::from_unpremultiplied_argb(x, rc, gc, bc);
                        buf[i] = (u32::from(s.a) << 24)
                            | (u32::from(s.r) << 16)
                            | (u32::from(s.g) << 8)
                            | u32::from(s.b);
                    }

                    let img = raqote::Image {
                        width: g.width as i32,
                        height: g.height as i32,
                        data: &buf[..],
                    };

                    self.draw_image_with_size_at(
                        g.width as f32,
                        g.height as f32,
                        g.x,
                        g.y,
                        &img,
                        &DrawOptions::default(),
                    );
                }
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

        dt.paint_params(params, &Default::default());

        // dt.write_png("/tmp/ye.png").expect("write failed");
    }

    #[test]
    fn test_visualize() {
        use minidx_core::layers as l;
        let network = (
            l::Dense::<f32, 2, 3>::default(),
            l::Bias1d::<f32, 3>::default(),
            l::Dense::<f32, 3, 1>::default(),
        );
        let mut dt = DrawTarget::new(460, 500);
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xcf, 0xcf, 0xcf,
        ));

        let params = ParamVisOpts::default();

        use VisualizableNetwork;
        network.visualize(&mut dt, params);
        dt.write_png("/tmp/ye.png").expect("write failed");
    }
}
