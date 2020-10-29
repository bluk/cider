// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! CloudKit stores data in the cloud.
//!
//! See [official documentation][apple_docs].
//!
//! [apple_docs]:
//! https://developer.apple.com/library/archive/documentation/DataManagement/Conceptual/CloudKitWebServicesReference/index.html#//apple_ref/doc/uid/TP40015240-CH41-SW1

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{format, string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{format, string::String, vec::Vec};

/// X-Apple-CloudKit-Request-KeyID HTTP header
pub const X_APPLE_CLOUDKIT_REQUEST_KEY_ID_HEADER: &str = "X-Apple-CloudKit-Request-KeyID";
/// X-Apple-CloudKit-Request-ISO8601Date HTTP header
pub const X_APPLE_CLOUDKIT_REQUEST_ISO8601_DATE_HEADER: &str =
    "X-Apple-CloudKit-Request-ISO8601Date";
/// X-Apple-CloudKit-Request-SignatureV1 HTTP header
pub const X_APPLE_CLOUDKIT_REQUEST_SIGNATURE_V1_HEADER: &str =
    "X-Apple-CloudKit-Request-SignatureV1";

/// Default base URL.
pub const BASE_URL: &str = "https://api.apple-cloudkit.com";

/// CloudKit container.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Container<'a>(pub &'a str);

/// Environment
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub enum Env {
    /// Development
    Dev,
    /// Production
    Prod,
}

impl Env {
    // TODO: Implement AsRef types and the like
    pub fn as_str(&self) -> &str {
        match self {
            Env::Dev => "development",
            Env::Prod => "production",
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct DbBasePath<'a> {
    container: &'a Container<'a>,
    env: Env,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> DbBasePath<'a> {
    pub fn new(container: &'a Container, env: Env) -> Self {
        Self { container, env }
    }

    pub fn version(&self) -> &str {
        "1"
    }

    pub fn container(&self) -> &Container {
        &self.container
    }

    pub fn environment(&self) -> &Env {
        &self.env
    }

    pub fn sub_path(&self) -> String {
        format!(
            "/database/{}/{}/{}",
            self.version(),
            self.container().0,
            self.environment().as_str()
        )
    }
}

/// Database
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub enum Db {
    Public,
    Private,
    Shared,
}

impl Db {
    // TODO: Implement AsRef types and the like
    pub fn as_str(&self) -> &str {
        match self {
            Db::Public => "public",
            Db::Private => "private",
            Db::Shared => "shared",
        }
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
pub trait Request<B>
where
    B: Serialize,
{
    fn sub_path(&self) -> String;

    fn body(&self) -> Option<&B>;
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct Operation {
    pub operation_type: String,
    pub record: Record,
    pub desired_keys: Option<Vec<String>>,
}

pub enum OperationType {
    Create,
    Update,
    ForceUpdate,
    Replace,
    ForceReplace,
    Delete,
    ForceDelete,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct Record {
    pub record_name: String,
    pub record_type: Option<String>,
    pub record_change_tag: Option<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct Zone {
    pub zone_id: ZoneId,
    pub sync_token: Option<Vec<u8>>,
    pub atomic: Option<bool>,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct ZoneId {
    pub zone_name: String,
    pub owner_record_name: Option<String>,
}

// #[cfg(any(feature = "alloc", feature = "std"))]
// impl ZoneId {
//     pub fn new(zone_name: &str, owner_record_name: Option<&str>) -> Self {
//         ZoneId {
//             zone_name: String::from(zone_name),
//             owner_record_name: owner_record_name.map(|s| String::from(s)),
//         }
//     }
// }
//

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod records;
#[cfg(any(feature = "alloc", feature = "std"))]
pub mod zones;
