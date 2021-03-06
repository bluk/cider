// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! DeviceCheck allow bits of data to be set and queried.
//!
//! See [official documentation][apple_docs].
//!
//! [apple_docs]: https://developer.apple.com/documentation/devicecheck

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

use crate::time::DurationSinceEpoch;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Env {
    Dev,
    Prod,
}

impl Env {
    fn base_endpoint(self) -> String {
        match self {
            Env::Dev => String::from("https://api.development.devicecheck.apple.com"),
            Env::Prod => String::from(" https://api.devicecheck.apple.com"),
        }
    }

    pub fn validate_device_endpoint(self) -> String {
        let mut s = self.base_endpoint();
        s.push_str("/v1/validate_device_token");
        s
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ValidationReq {
    device_token: String,
    transaction_id: String,
    timestamp: u64,
}

impl ValidationReq {
    pub fn new<T>(device_token: &str, transaction_id: &str, duration_since_epoch: T) -> Self
    where
        T: DurationSinceEpoch,
    {
        ValidationReq {
            device_token: String::from(device_token),
            transaction_id: String::from(transaction_id),
            timestamp: duration_since_epoch.as_millis(),
        }
    }
}

// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub enum VerifyDeviceTokenResult {
//     Verified,
//     BadRequest,
//     Unauthorized,
//     Forbidden,
//     TooManyRequests,
//     InternalServerError,
//     Unknown,
// }
