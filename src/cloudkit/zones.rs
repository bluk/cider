// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

use super::{Db, DbBasePath, Request};

#[cfg(any(feature = "alloc", feature = "std"))]
pub struct FetchZonesRequest<'a> {
    db_base_path: DbBasePath<'a>,
    db: Db,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> FetchZonesRequest<'a> {
    pub fn new(db_base_path: DbBasePath<'a>, db: Db) -> Self {
        Self { db_base_path, db }
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> Request<core::marker::PhantomData<u8>> for FetchZonesRequest<'a> {
    fn sub_path(&self) -> String {
        let mut p = self.db_base_path.sub_path();
        p.push('/');
        p.push_str(self.db.as_str());
        p.push_str("/zones/list");
        p
    }

    fn body(&self) -> Option<&core::marker::PhantomData<u8>> {
        None
    }
}

use serde_json::{Map, Value};

#[cfg(any(feature = "alloc", feature = "std"))]
pub struct FetchZonesResponse {
    pub value: Map<String, serde_json::Value>,
}

impl serde::Serialize for FetchZonesResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for FetchZonesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map: Map<String, Value> = Map::deserialize(deserializer)?;
        Ok(Self { value: map })
    }
}
