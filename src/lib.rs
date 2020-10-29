// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A helper library for Apple services.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod cloudkit;
pub mod crypto;
#[cfg(any(feature = "alloc", feature = "std"))]
pub mod device_check;
#[cfg(any(feature = "alloc", feature = "std"))]
pub mod siwa;
pub mod time;

/// The 10 character team ID.
///
/// It is used as the issuer value in the claims payload of a JWT.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TeamId<'a>(pub &'a str);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
