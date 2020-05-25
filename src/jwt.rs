// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Helpers for JSON Web Tokens.
//!
//! [JSON Web Tokens][rfc_7519] are used for authentication. See the [general usage
//! documentation][apple_docs].
//!
//! [rfc_7519]: https://tools.ietf.org/html/rfc7519
//! [apple_docs]:
//! https://developer.apple.com/library/archive/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/CommunicatingwithAPNs.html#//apple_ref/doc/uid/TP40008194-CH11-SW1

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::str::FromStr;
#[cfg(feature = "std")]
use std::str::FromStr;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::{time::DurationSinceEpoch, TeamId};
use serde::{Deserialize, Serialize};

/// The key ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KeyId<'a>(pub &'a str);

/// Algorithm used to sign the JWT.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Algorithm {
    Es256,
    Rs256,
}

impl Algorithm {
    fn as_str(&self) -> &str {
        match self {
            Algorithm::Es256 => "ES256",
            Algorithm::Rs256 => "RS256",
        }
    }
}

/// Errors for when parsing a str into an algorithm.
#[non_exhaustive]
pub enum ParseAlgorithmError {
    UnknownAlgorithm,
}

impl FromStr for Algorithm {
    type Err = ParseAlgorithmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ES256" => Ok(Algorithm::Es256),
            "RS256" => Ok(Algorithm::Rs256),
            _ => Err(ParseAlgorithmError::UnknownAlgorithm),
        }
    }
}

/// Type of the data in the key.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum KeyData<'a> {
    Pkcs8(&'a [u8]),
    Jwk(&'a Jwk),
}

/// Key used to sign a JWT.
pub struct Key<'a> {
    id: &'a KeyId<'a>,
    alg: Algorithm,
    data: &'a KeyData<'a>,
}

impl<'a> Key<'a> {
    /// Constructs a new signing key.
    ///
    /// The signing key id is the associated identifier for the key. The algorithm is the intended
    /// algorithm used to sign data with this key. The data is the raw key bytes (e.g. the contents
    /// of the *.p8 file).
    pub fn new(id: &'a KeyId<'a>, alg: Algorithm, data: &'a KeyData<'a>) -> Self {
        Key { id, alg, data }
    }

    /// Returns the key ID.
    pub fn id(&self) -> &KeyId {
        self.id
    }

    /// Returns the intended signing algorithm.
    pub fn alg(&self) -> Algorithm {
        self.alg
    }

    /// Returns the signing key in raw bytes.
    pub fn data(&self) -> &KeyData {
        self.data
    }
}

/// Contains the algorithm and the key ID used to sign the JWT.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Header<'a> {
    alg: &'a str,
    kid: &'a str,
}

impl<'a> Header<'a> {
    /// Constructs a `Header` from a signing key.
    pub fn new(key: &'a Key<'a>) -> Self {
        Header {
            alg: key.alg.as_str(),
            kid: key.id.0,
        }
    }

    /// Returns the signing algorithm.
    pub fn alg(&self) -> &str {
        self.alg
    }

    /// Returns the key ID.
    pub fn kid(&self) -> &str {
        self.kid
    }
}

/// Contains the issuer ID (team ID), when the token was issued, and when the token expires.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Claims<'a> {
    pub iss: &'a str,
    pub iat: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<&'a str>,
}

impl<'a> Claims<'a> {
    /// Constructs a `Claims`.
    pub fn new<T>(team_id: &'a TeamId, duration_since_epoch: T) -> Self
    where
        T: DurationSinceEpoch,
    {
        Claims {
            iss: team_id.0,
            iat: duration_since_epoch.as_secs(),
            exp: None,
            aud: None,
            sub: None,
        }
    }

    /// Returns the issuer of the token (usually the team ID).
    pub fn iss(&self) -> &str {
        self.iss
    }

    /// Returns when the token was issued as the number of seconds since the Unix epoch.
    pub fn iat(&self) -> u64 {
        self.iat
    }

    /// Returns when the token should expire as the number of seconds since the Unix epoch.
    pub fn exp(&self) -> Option<u64> {
        self.exp
    }

    /// Returns the intended audience.
    pub fn aud(&self) -> Option<&str> {
        self.aud
    }

    /// Returns the subject.
    pub fn sub(&self) -> Option<&str> {
        self.sub
    }
}

/// A JSON Web Key.
///
/// See [RFC 7517][rfc_7517].
///
/// [rfc_7517]: https://tools.ietf.org/html/rfc7517
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}
