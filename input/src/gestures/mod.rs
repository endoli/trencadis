// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gestures
//!
//! A gesture provides a way to map underlying input events to higher
//! level actions within a user interface. These can be used rather than
//! the low level events to provide a mechanism where users can customize
//! which events result in a corresponding action.

use events::Event;

#[allow(missing_docs)]
pub trait Gesture {
    fn matches_event(&self, _event: &Event) -> bool;
}

#[allow(missing_docs)]
#[derive(Default)]
pub struct GestureMapping<'gm> {
    #[allow(dead_code)]
    gestures: Vec<&'gm Gesture>,
}

impl<'gm> GestureMapping<'gm> {
    #[allow(missing_docs)]
    pub fn lookup_gesture(&self, event: &Event) -> Option<&'gm Gesture> {
        self.gestures
            .iter()
            .filter(|g| g.matches_event(event))
            .last()
            .map(|sg| *sg)
    }
}
