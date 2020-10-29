// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

use super::{Db, DbBasePath, Operation, Request, ZoneId};
use serde::Serialize;

#[derive(Clone, Debug, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct ModifyRecordsRequestBody {
    pub operations: Vec<Operation>,
    pub zone_id: ZoneId,
    pub atomic: Option<bool>,
    pub desired_keys: Option<Vec<String>>,
    pub numbers_as_strings: Option<bool>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
pub struct ModifyRecordsRequest<'a> {
    db_base_path: DbBasePath<'a>,
    db: Db,
    body: ModifyRecordsRequestBody,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> ModifyRecordsRequest<'a> {
    pub fn new(db_base_path: DbBasePath<'a>, db: Db, body: ModifyRecordsRequestBody) -> Self {
        Self {
            db_base_path,
            db,
            body,
        }
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> Request<ModifyRecordsRequestBody> for ModifyRecordsRequest<'a> {
    fn sub_path(&self) -> String {
        let mut p = self.db_base_path.sub_path();
        p.push('/');
        p.push_str(self.db.as_str());
        p.push_str("/records/modify");
        p
    }

    fn body(&self) -> Option<&ModifyRecordsRequestBody> {
        Some(&self.body)
    }
}
