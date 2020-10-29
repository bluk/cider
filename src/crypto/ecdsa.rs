// Copyright 2020 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Algorithm for signing data.
pub trait EcdsaSigningAlgorithm: super::SigningAlgorithm {}

/// ECDSA signature using P-256 and SHA-256 in the ASN.1 format.
pub struct EcdsaP256Sha256Asn1Format {}

impl EcdsaSigningAlgorithm for EcdsaP256Sha256Asn1Format {}
impl super::SigningAlgorithm for EcdsaP256Sha256Asn1Format {}

/// ECDSA signature using P-256 and SHA-256 in the fixed format.
pub struct EcdsaP256Sha256FixedFormat {}

impl EcdsaSigningAlgorithm for EcdsaP256Sha256FixedFormat {}
impl super::SigningAlgorithm for EcdsaP256Sha256FixedFormat {}
