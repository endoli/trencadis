// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use app_units::Au;
use rusttype::{Font, FontCollection, Point, Scale};
use webrender_traits::{LayoutPoint, LayoutRect, LayoutSize};
use webrender_traits::{ColorF, DisplayListBuilder};
use webrender_traits::{FontKey, GlyphInstance, RenderApi};

pub struct Transcript<'t> {
    font: Font<'t>,
    font_key: FontKey,
    rect: LayoutRect,
    entries: Vec<Entry>,
}

impl<'t> Transcript<'t> {
    pub fn new(api: &RenderApi, rect: LayoutRect) -> Self {
        let font_bytes = include_bytes!("res/Inconsolata-Regular.ttf");
        let font = FontCollection::from_bytes(font_bytes as &[u8]).into_font().unwrap();
        let font_key = api.generate_font_key();
        api.add_raw_font(font_key, font_bytes.to_vec(), 0);
        Transcript {
            font: font,
            font_key: font_key,
            rect: rect,
            entries: vec![],
        }
    }

    pub fn new_with_entries(api: &RenderApi, rect: LayoutRect, entries: Vec<Entry>) -> Self {
        let font_bytes = include_bytes!("res/Inconsolata-Regular.ttf");
        let font = FontCollection::from_bytes(font_bytes as &[u8]).into_font().unwrap();
        let font_key = api.generate_font_key();
        api.add_raw_font(font_key, font_bytes.to_vec(), 0);
        Transcript {
            font: font,
            font_key: font_key,
            rect: rect,
            entries: entries,
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
        let clip = builder.push_clip_region(&self.rect, Vec::new(), None);
        builder.push_rect(self.rect, clip, ColorF::new(0.0, 1.0, 0.0, 0.3));
        for (idx, entry) in self.entries.iter().enumerate() {
            let r = LayoutRect::new(LayoutPoint::new(self.rect.min_x() + 10.0,
                                                     self.rect.min_y() + 10.0 +
                                                     60.0 * (idx as f32)),
                                    LayoutSize::new(100.0, 20.0));
            self.render_entry(entry, builder, r);
        }
    }

    pub fn render_entry(&self, e: &Entry, builder: &mut DisplayListBuilder, rect: LayoutRect) {
        let black = ColorF::new(0.0, 0.0, 0.0, 1.0);
        let r = LayoutRect::new(LayoutPoint::new(rect.min_x(), rect.min_y()),
                                LayoutSize::new(50.0, rect.max_y() - rect.min_y()));
        self.draw_text(builder, &e.prompt, r, black);
        let r = LayoutRect::new(LayoutPoint::new(rect.min_x() + 60.0, rect.min_y()),
                                LayoutSize::new(rect.max_x() - rect.min_y() - 60.0, 30.0));
        self.draw_text(builder, &e.input, r, black);
        let r = LayoutRect::new(LayoutPoint::new(rect.min_x() + 60.0, rect.min_y() + 30.0),
                                LayoutSize::new(50.0, rect.max_y() - rect.min_y() - 30.0));
        self.draw_text(builder, &e.output, r, black);
    }

    fn draw_text(&self,
                 builder: &mut DisplayListBuilder,
                 text: &str,
                 rect: LayoutRect,
                 fgcolor: ColorF) {
        let font_size = 14.0;
        let glyphs = self.font
            .layout(text,
                    Scale::uniform(font_size),
                    Point {
                        x: rect.min_x(),
                        y: rect.min_y() + 20.0,
                    })
            .map(|glyph| {
                     GlyphInstance {
                         index: glyph.id().0,
                         point: LayoutPoint::new(glyph.position().x, glyph.position().y),
                     }
                 })
            .collect::<Vec<_>>();
        let clip = builder.push_clip_region(&self.rect, Vec::new(), None);
        builder.push_text(rect,
                          clip,
                          &glyphs,
                          self.font_key,
                          fgcolor,
                          Au::from_f32_px(font_size),
                          Au::from_px(0),
                          None);
    }
}

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
            prompt: prompt,
            input: input,
            output: output,
            elapsed: 0.0,
            complete: false,
        }
    }

    pub fn complete(&mut self) {
        self.elapsed = 3.0;
        self.complete = true;
    }
}
