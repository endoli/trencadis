// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Trencadis Input
//!
//! The hope is that this quickly becomes a stand-alone crate
//! and can be widely used.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unstable_features,
        unused_import_braces, unused_qualifications)]

pub mod events;
pub mod gestures;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
