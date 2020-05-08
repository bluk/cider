// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Sign in with Apple allows single sign-on using an Apple ID.
//!
//! See [official documentation][apple_docs].
//!
//! [apple_docs]: https://developer.apple.com/documentation/sign_in_with_apple

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(any(feature = "alloc", feature = "std"))]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct JWKSet {
    pub keys: Vec<JWK>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct JWK {
    pub kty: String,
    pub kid: String,
    pub r#use: String,
    pub alg: String,
    pub e: String,
    pub n: String,
}

/// The response from the authorization redirect.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct AuthResponse {
    pub code: Option<String>,
    pub id_token: Option<String>,
    pub state: Option<String>,
    pub user: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct User {
    pub email: Option<String>,
    pub name: Option<Name>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct Name {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
}

/// The id_token value's JWT claims.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct IdTokenClaims {
    pub iss: Option<String>,
    pub aud: Option<String>,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
    pub sub: Option<String>,
    pub c_hash: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<String>,
    pub is_private_email: Option<String>,
    pub auth_time: Option<u64>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct ValidateAuthCodeRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg(any(feature = "alloc", feature = "std"))]
pub struct TokenResponse {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub error: Option<String>,
}
