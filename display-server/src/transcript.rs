// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use app_units::Au;
use rusttype::{Font, FontCollection, Point, Scale};
use webrender::api::*;

pub struct Transcript<'t> {
    font: Font<'t>,
    font_key: FontKey,
    font_instance_key: FontInstanceKey,
    layout_info: LayoutPrimitiveInfo,
    entries: Vec<Entry>,
}

impl<'t> Transcript<'t> {
    pub fn new(
        api: &RenderApi,
        resources: &mut ResourceUpdates,
        layout_info: LayoutPrimitiveInfo,
    ) -> Self {
        let font_bytes = include_bytes!("res/Inconsolata-Regular.ttf");
        let font = FontCollection::from_bytes(font_bytes as &[u8])
            .into_font()
            .unwrap();
        let font_key = api.generate_font_key();
        resources.add_raw_font(font_key, font_bytes.to_vec(), 0);

        let font_instance_key = api.generate_font_instance_key();
        resources.add_font_instance(
            font_instance_key,
            font_key,
            Au::from_px(14),
            None,
            None,
            Vec::new(),
        );

        Transcript {
            font,
            font_key,
            font_instance_key,
            layout_info,
            entries: vec![],
        }
    }

    pub fn new_with_entries(
        api: &RenderApi,
        resources: &mut ResourceUpdates,
        layout_info: LayoutPrimitiveInfo,
        entries: Vec<Entry>,
    ) -> Self {
        let font_bytes = include_bytes!("res/Inconsolata-Regular.ttf");
        let font = FontCollection::from_bytes(font_bytes as &[u8])
            .into_font()
            .unwrap();
        let font_key = api.generate_font_key();
        resources.add_raw_font(font_key, font_bytes.to_vec(), 0);
        let font_instance_key = api.generate_font_instance_key();
        resources.add_font_instance(
            font_instance_key,
            font_key,
            Au::from_px(14),
            None,
            None,
            Vec::new(),
        );
        Transcript {
            font,
            font_key,
            font_instance_key,
            layout_info,
            entries,
        }
    }

    pub fn append_entry(&mut self, entry: Entry) -> usize {
        self.entries.push(entry);
        self.entries.len()
    }

    pub fn complete_entry(&mut self, entry_idx: usize) {
        self.entries.get_mut(entry_idx).map(|e| e.complete());
    }

    pub fn render(&self, builder: &mut DisplayListBuilder) {
        builder.push_rect(&self.layout_info, ColorF::new(0.0, 1.0, 0.0, 0.3));
        for (idx, entry) in self.entries.iter().enumerate() {
            let r = LayoutPrimitiveInfo::new(LayoutRect::new(
                LayoutPoint::new(
                    self.layout_info.rect.min_x() + 10.0,
                    self.layout_info.rect.min_y() + 10.0 + 60.0 * (idx as f32),
                ),
                LayoutSize::new(100.0, 20.0),
            ));
            self.render_entry(entry, builder, r);
        }
    }

    pub fn render_entry(
        &self,
        e: &Entry,
        builder: &mut DisplayListBuilder,
        layout_info: LayoutPrimitiveInfo,
    ) {
        let black = ColorF::new(0.0, 0.0, 0.0, 1.0);
        let r = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(
                layout_info.rect.min_x(),
                layout_info.rect.min_y(),
            ),
            LayoutSize::new(
                50.0,
                layout_info.rect.max_y() - layout_info.rect.min_y(),
            ),
        ));
        self.draw_text(builder, &e.prompt, r, black);
        let r = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(
                layout_info.rect.min_x() + 60.0,
                layout_info.rect.min_y(),
            ),
            LayoutSize::new(
                layout_info.rect.max_x() - layout_info.rect.min_y() - 60.0,
                30.0,
            ),
        ));
        self.draw_text(builder, &e.input, r, black);
        let r = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(
                layout_info.rect.min_x() + 60.0,
                layout_info.rect.min_y() + 30.0,
            ),
            LayoutSize::new(
                50.0,
                layout_info.rect.max_y() - layout_info.rect.min_y() - 30.0,
            ),
        ));
        self.draw_text(builder, &e.output, r, black);
    }

    fn draw_text(
        &self,
        builder: &mut DisplayListBuilder,
        text: &str,
        layout_info: LayoutPrimitiveInfo,
        fgcolor: ColorF,
    ) {
        let font_size = 14;
        let glyphs = self.font
            .layout(
                text,
                Scale::uniform(font_size as f32),
                Point {
                    x: layout_info.rect.min_x(),
                    y: layout_info.rect.min_y() + 20.0,
                },
            )
            .map(|glyph| {
                GlyphInstance {
                    index: glyph.id().0,
                    point: LayoutPoint::new(glyph.position().x, glyph.position().y),
                }
            })
            .collect::<Vec<_>>();
        builder.push_text(&layout_info, &glyphs, self.font_instance_key, fgcolor, None);
    }
}

#[derive(Default)]
pub struct Entry {
    prompt: String,
    input: String,
    output: String,
    elapsed: f64,
    complete: bool,
}

impl Entry {
    pub fn new(prompt: String, input: String, output: String) -> Entry {
        Entry {
            prompt,
            input,
            output,
            ..Default::default()
        }
    }

    pub fn complete(&mut self) {
        self.elapsed = 3.0;
        self.complete = true;
    }
}
