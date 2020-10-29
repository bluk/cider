// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Cryptography used in services.

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

/// An identifier for a key used in signing/encrypting data.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct KeyId<'a>(pub &'a str);

/// Bytes in PKCS8 format.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pkcs8Key<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for Pkcs8Key<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

/// A signing algorithm.
pub trait SigningAlgorithm {}

use serde::Deserialize;

/// A JSON Web Key.
///
/// See [RFC 7517][rfc_7517].
///
/// [rfc_7517]: https://tools.ietf.org/html/rfc7517
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct Jwk {
    pub kty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,

    // EC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,

    // RSA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
}

/// A JSON Web Key set.
///
/// See [RFC 7517][rfc_7517].
///
/// [rfc_7517]: https://tools.ietf.org/html/rfc7517
#[derive(Clone, Debug, Hash, PartialEq, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}

pub mod ecdsa;
pub mod jwt;
