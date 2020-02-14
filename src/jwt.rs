// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JSON Web Tokens used for auth.
//!
//! See the [usage documentation][apple_docs].
//!
//! [apple_docs]:
//! https://developer.apple.com/library/archive/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/CommunicatingwithAPNs.html#//apple_ref/doc/uid/TP40008194-CH11-SW1

use crate::TeamId;
use serde::Serialize;
use std::time::Duration;

/// Algorithm used to sign the JWT.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Algorithm<'a> {
    ES256,
    Unknown(&'a str),
}

impl<'a> Algorithm<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Algorithm::ES256 => "ES256",
            Algorithm::Unknown(other) => other,
        }
    }
}

impl<'a> From<&'a str> for Algorithm<'a> {
    fn from(other: &'a str) -> Self {
        match other {
            "ES256" => Algorithm::ES256,
            _ => Algorithm::Unknown(other),
        }
    }
}

/// Key used to sign a JWT.
pub struct SigningKey<'a> {
    id: &'a str,
    alg: Algorithm<'a>,
    data: &'a [u8],
}

impl<'a> SigningKey<'a> {
    pub fn new(id: &'a str, alg: Algorithm<'a>, data: &'a [u8]) -> Self {
        SigningKey { id, alg, data }
    }

    pub fn id(&self) -> &str {
        self.id
    }

    pub fn alg(&self) -> Algorithm<'a> {
        self.alg
    }

    pub fn data(&self) -> &[u8] {
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
    pub fn new(key: &'a SigningKey<'a>) -> Self {
        Header {
            alg: key.alg.as_str(),
            kid: key.id,
        }
    }

    pub fn alg(&self) -> Algorithm {
        Algorithm::from(self.alg)
    }

    pub fn kid(&self) -> &str {
        &self.kid
    }
}

/// Contains the issuer ID (team ID) and when the JWT was issued.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Claims<'a> {
    iss: &'a str,
    iat: u64,
}

impl<'a> Claims<'a> {
    pub fn new(team_id: &'a TeamId, duration_since_epoch: Duration) -> Self {
        Claims {
            iss: team_id.0,
            iat: duration_since_epoch.as_secs(),
        }
    }

    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn iat(&self) -> u64 {
        self.iat
    }
}

// /// Signs the header and claims for a JWT.
// pub trait Signer {
//     /// Returns an encoded and signed JWT.
//     fn encode_and_sign<'a, 'b>(
//         &self,
//         header: &Header<'a>,
//         claims: &Claims<'b>,
//     ) -> Result<String, Box<dyn std::error::Error>>;
// }
