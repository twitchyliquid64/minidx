use fontdue::layout::{GlyphPosition, Layout, LayoutSettings, TextStyle};
use fontdue::Font;

use raqote::{DrawOptions, DrawTarget, SolidSource};

/// A loaded font for use in rastering text during visualization.
pub struct VisFont {
    layout: Layout<()>,
    font: std::sync::Arc<Font>,
}

impl Clone for VisFont {
    fn clone(&self) -> Self {
        Self {
            layout: Layout::<()>::new(fontdue::layout::CoordinateSystem::PositiveYDown),
            font: self.font.clone(),
        }
    }
}

impl std::fmt::Debug for VisFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VisFont")
            .field("layout", &())
            .field("font", &self.font)
            .finish()
    }
}

impl VisFont {
    pub fn default_font() -> Option<Self> {
        use rust_fontconfig::{FcFontCache, FcPattern};
        let cache = FcFontCache::build();
        // for font in cache.list() {
        //     println!("{:?}", font);
        // }

        let candidates = [
            FcPattern {
                family: Some(String::from("Liberation Mono")),
                ..Default::default()
            },
            FcPattern {
                family: Some(String::from("DejaVu Sans Mono")),
                ..Default::default()
            },
            FcPattern {
                family: Some(String::from("Andale Mono")),
                ..Default::default()
            },
            FcPattern {
                family: Some(String::from("FreeMono")),
                ..Default::default()
            },
            FcPattern {
                family: Some(String::from("Liberation Mono")),
                ..Default::default()
            },
            FcPattern {
                family: Some(String::from("FreeSans")),
                ..Default::default()
            },
            FcPattern {
                name: Some(String::from("Arial")),
                ..Default::default()
            },
        ];

        for c in candidates.iter() {
            let result = cache
                .query(c)
                .map(|p| (p.font_index, std::fs::read(p.path.clone()).unwrap()));
            if let Some((idx, font_bytes)) = result {
                return Some(Self {
                    layout: Layout::<()>::new(fontdue::layout::CoordinateSystem::PositiveYDown),
                    font: std::sync::Arc::new(
                        Font::from_bytes(
                            font_bytes,
                            fontdue::FontSettings {
                                collection_index: idx as u32,
                                ..Default::default()
                            },
                        )
                        .unwrap(),
                    ),
                });
            }
        }

        None
    }

    pub fn layout_str(
        &mut self,
        layout_settings: &LayoutSettings,
        text: &str,
        font_size: f32,
    ) -> impl Iterator<Item = (Vec<u8>, &GlyphPosition)> {
        self.layout.reset(layout_settings);
        self.layout.append(
            &[self.font.clone()], // its an arc
            &TextStyle::new(text, font_size, 0),
        );
        self.layout
            .glyphs()
            .iter()
            .map(|g| (self.font.rasterize_config(g.key).1, g))
    }

    pub fn raster(
        &mut self,
        layout_settings: &LayoutSettings,
        text: &str,
        font_size: f32,
        rgb: (u8, u8, u8),
        dt: &mut DrawTarget,
    ) {
        let glyphs = self.layout_str(layout_settings, text, font_size);

        let (rc, gc, bc) = rgb;
        for (b, g) in glyphs {
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

            dt.draw_image_with_size_at(
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
