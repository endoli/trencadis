// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Events
//!
//! This module is based on the specifications provided by the
//! W3C for web browsers.

mod keyboard;
mod pointer;

pub use self::keyboard::*;
pub use self::pointer::*;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventType {
    Pointer,
    Keyboard,
}

#[allow(missing_docs)]
#[allow(dead_code)]
pub struct Event {
    event_type: EventType,
    keyboard_event_data: Option<KeyboardEventData>,
    pointer_event_data: Option<PointerEventData>,
}
