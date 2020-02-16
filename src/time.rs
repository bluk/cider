// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Helpers for time.

/// A type which can return the duration since the epoch.
pub trait DurationSinceEpoch {
    /// Returns the number of seconds since the epoch.
    fn as_secs(&self) -> u64;

    /// Returns the number of milliseconds since the epoch.
    fn as_millis(&self) -> u64;
}

#[cfg(feature = "std")]
use std::convert::From;
#[cfg(feature = "std")]
use std::time::{Duration, SystemTime};

/// Duration since the epoch represented by standard library types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg(feature = "std")]
pub struct StdDurationSinceEpoch(Duration);

#[cfg(feature = "std")]
impl StdDurationSinceEpoch {
    /// Returns the current duration since the epoch.
    pub fn now() -> Self {
        StdDurationSinceEpoch(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("SystemTime should be later than UNIX_EPOCH."),
        )
    }
}

#[cfg(feature = "std")]
impl DurationSinceEpoch for StdDurationSinceEpoch {
    fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    fn as_millis(&self) -> u64 {
        self.0.as_secs() * 1000 + u64::from(self.0.subsec_millis())
    }
}
