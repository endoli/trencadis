// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use webrender::api::{BorderDetails, BorderWidths};
use webrender::api::{ColorF, DisplayListBuilder};
use webrender::api::LayoutRect;

pub trait Item {
    fn build(&self, builder: &mut DisplayListBuilder);
}

pub struct RectItem {
    pub rect: LayoutRect,
    pub color: ColorF,
}

impl Item for RectItem {
    fn build(&self, builder: &mut DisplayListBuilder) {
        builder.push_rect(self.rect, self.rect, self.color);
    }
}

pub struct BorderItem {
    pub rect: LayoutRect,
    pub widths: BorderWidths,
    pub details: BorderDetails,
}

impl Item for BorderItem {
    fn build(&self, builder: &mut DisplayListBuilder) {
        builder.push_border(self.rect, self.rect, self.widths, self.details);
    }
}

#[derive(Default)]
pub struct Frame {
    items: Vec<Box<Item>>,
    children: Vec<Frame>,
}

impl Frame {
    pub fn build(&self, builder: &mut DisplayListBuilder) {
        for item in &self.items {
            item.build(builder);
        }
        for child in &self.children {
            child.build(builder);
        }
    }

    pub fn push_child(&mut self, frame: Frame) {
        self.children.push(frame);
    }

    pub fn push_rect(&mut self, rect: LayoutRect, color: ColorF) {
        self.items.push(Box::new(RectItem {
            rect: rect,
            color: color,
        }));
    }
}
