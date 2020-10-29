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

use crate::{crypto::KeyId, time::DurationSinceEpoch, TeamId};
use serde::Serialize;

pub trait Key<'a> {
    /// Returns the key ID.
    fn id(&self) -> &KeyId<'a>;

    /// Returns the intended signing algorithm.
    fn alg() -> &'static str;
}

/// Contains the algorithm and the key ID used to sign the JWT.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header<'a> {
    alg: &'static str,
    kid: &'a str,
}

impl<'a> Header<'a> {
    /// Constructs a `Header` from a signing key.
    pub fn new<K>(key: &'a K) -> Self
    where
        K: Key<'a>,
    {
        Header {
            alg: K::alg(),
            kid: key.id().0,
        }
    }

    /// Returns the signing algorithm.
    pub fn alg(&self) -> &'static str {
        self.alg
    }

    /// Returns the key ID.
    pub fn kid(&self) -> &str {
        self.kid
    }
}

impl<'a> Serialize for Header<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("alg", self.alg)?;
        map.serialize_entry("kid", self.kid)?;
        map.end()
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
