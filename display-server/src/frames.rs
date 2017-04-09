// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use webrender_traits::{BorderDetails, BorderWidths};
use webrender_traits::{ClipRegion, ColorF, DisplayListBuilder};
use webrender_traits::{LayoutRect, MixBlendMode, ScrollPolicy, TransformStyle};

pub trait Item {
    fn build(&self, builder: &mut DisplayListBuilder);
}

pub struct RectItem {
    pub rect: LayoutRect,
    pub clip: ClipRegion,
    pub color: ColorF,
}

impl Item for RectItem {
    fn build(&self, builder: &mut DisplayListBuilder) {
        builder.push_rect(self.rect, self.clip, self.color);
    }
}

pub struct BorderItem {
    pub rect: LayoutRect,
    pub clip: ClipRegion,
    pub widths: BorderWidths,
    pub details: BorderDetails,
}

impl Item for BorderItem {
    fn build(&self, builder: &mut DisplayListBuilder) {
        builder.push_border(self.rect, self.clip, self.widths, self.details);
    }
}

pub struct Frame {
    bounds: LayoutRect,
    items: Vec<Box<Item>>,
    children: Vec<Frame>,
}

impl Frame {
    pub fn new(bounds: LayoutRect) -> Self {
        Frame {
            bounds: bounds,
            items: vec![],
            children: vec![],
        }
    }

    pub fn build(&self, builder: &mut DisplayListBuilder) {
        builder.push_stacking_context(ScrollPolicy::Scrollable,
                                      self.bounds,
                                      0,
                                      None,
                                      TransformStyle::Flat,
                                      None,
                                      MixBlendMode::Normal,
                                      Vec::new());
        for item in &self.items {
            item.build(builder);
        }
        for child in &self.children {
            child.build(builder);
        }
        builder.pop_stacking_context();
    }

    pub fn push_child(&mut self, frame: Frame) {
        self.children.push(frame);
    }

    pub fn push_rect(&mut self, rect: LayoutRect, clip: ClipRegion, color: ColorF) {
        self.items.push(Box::new(RectItem {
                                     rect: rect,
                                     clip: clip,
                                     color: color,
                                 }));
    }
}
