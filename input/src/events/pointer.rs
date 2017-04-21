// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(missing_docs)]
pub enum PointerEventType {
    Unknown,
    Over,
    Enter,
    Down,
    Move,
    Up,
    Cancel,
    Out,
    Leave,
    GotCapture,
    LostCapture,
    Other(String),
}

#[allow(missing_docs)]
pub enum PointerType {
    Unknown,
    Mouse,
    Pen,
    Touch,
    Other(String),
}

#[allow(missing_docs)]
pub struct PointerEventData {
    pub event_type: PointerEventType,
    pub pointer_id: u32,
    pub width: f64,
    pub height: f64,
    pub pressure: f32,
    pub tangential_pressure: f32,
    pub tilt_x: u32,
    pub tilt_y: u32,
    pub twist: u32,
    pub pointer_type: PointerType,
    pub is_primary: bool,
}

impl Default for PointerEventData {
    fn default() -> Self {
        Self {
            event_type: PointerEventType::Unknown,
            pointer_id: 0,
            width: 1.0,
            height: 1.0,
            pressure: 0.0,
            tangential_pressure: 0.0,
            tilt_x: 0,
            tilt_y: 0,
            twist: 0,
            pointer_type: PointerType::Unknown,
            is_primary: false,
        }
    }
}
