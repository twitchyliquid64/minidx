use fontdue::layout::{Layout, LayoutSettings, TextStyle};
use fontdue::Font;
use minidx_core::Dtype;
use raqote::*;

#[derive(Debug, Clone)]
pub struct ParamBox {
    w: f32,
    h: f32,
}

impl Default for ParamBox {
    fn default() -> Self {
        Self { w: 40.0, h: 40.0 }
    }
}

/// How to scale the representation of parameters relative to each other.
#[derive(Debug, Clone)]
pub enum ParamScale {
    StdDev { mul: f32 },
}

impl Default for ParamScale {
    fn default() -> Self {
        Self::StdDev { mul: 1.2 }
    }
}

/// Options for rendering a set of parameters.
#[derive(Debug, Clone)]
pub struct ParamOpts {
    offset: (f32, f32),
    cell: ParamBox,
    scale: ParamScale,
    font: std::sync::Arc<Font>,
}

impl Default for ParamOpts {
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

/// Implements visual rendering of a set of parameters.
pub trait PaintParams<P> {
    fn paint_params(&mut self, params: &P, opts: &ParamOpts);
}

impl<E: Dtype, const L: usize> PaintParams<[E; L]> for DrawTarget {
    fn paint_params(&mut self, params: &[E; L], opts: &ParamOpts) {
        let mut layout = Layout::<()>::new(fontdue::layout::CoordinateSystem::PositiveYDown);

        let scale = 1.0;

        for (i, v) in params.iter().enumerate() {
            let tl = (opts.offset.0 + opts.cell.w * i as f32, opts.offset.1);

            // Make box
            let mut pb = PathBuilder::new();
            pb.move_to(tl.0, tl.1);
            pb.line_to(tl.0 + opts.cell.w, tl.1);
            pb.line_to(tl.0 + opts.cell.w, tl.1 + opts.cell.h);
            pb.line_to(tl.0, tl.1 + opts.cell.h);
            if i == 0 {
                pb.line_to(tl.0, tl.1);
            }
            let p = pb.finish();

            let v = scale * v.to_f32().unwrap();
            self.fill(
                &p,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(
                    0xFF,
                    ((-v).tanh().max(0.0) * 125.0) as u8 + 60,
                    (v.tanh().max(0.0) * 115.0) as u8 + 60,
                    60,
                )),
                &DrawOptions::new(),
            );
            self.stroke(
                &p,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(0xFF, 0, 0, 0)),
                &StrokeStyle {
                    width: 1.0,
                    ..StrokeStyle::default()
                },
                &DrawOptions::new(),
            );

            layout.reset(&LayoutSettings {
                x: tl.0,
                y: tl.1 + 1.0,
                max_width: Some(opts.cell.w),
                max_height: Some(opts.cell.h - 2.0),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                vertical_align: fontdue::layout::VerticalAlign::Middle,
                ..LayoutSettings::default()
            });
            let v_abs = v.abs();
            layout.append(
                &[opts.font.clone()], // its an arc
                &TextStyle::new(
                    if v_abs >= 10.0 {
                        format!("{:.0}", v_abs)
                    } else {
                        format!("{:.1}", v_abs)
                    }
                    .as_str(),
                    17.0,
                    0,
                ),
            );

            let (rc, gc, bc) = if v_abs > 0.45 {
                (195, 195, 195)
            } else {
                (10, 10, 10)
            };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paint_params() {
        let params = [
            -10.0, -1.0, -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1, 0.0, 0.1f32, 0.2,
            0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 10.0,
        ];
        let mut dt = DrawTarget::new(940, 200);
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xcf, 0xcf, 0xcf,
        ));

        dt.paint_params(&params, &Default::default());

        // dt.write_png("/tmp/ye.png");
    }
}
