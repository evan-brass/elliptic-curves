//! NIST P-384 elliptic curve (a.k.a. secp384r1)
//!
//! ## Minimum Supported Rust Version
//!
//! Rust **1.41** or higher.
//!
//! Minimum supported Rust version can be changed in the future, but it will be
//! done with a minor version bump.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/p384/0.3.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "ecdsa")]
#[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
pub mod ecdsa;

pub use elliptic_curve;

use elliptic_curve::consts::U48;

#[cfg(feature = "oid")]
use elliptic_curve::oid::ObjectIdentifier;

/// NIST P-384 elliptic curve.
///
/// This curve is also known as secp384r1 (SECG) and is specified in
/// FIPS 186-4: Digital Signature Standard (DSS):
///
/// <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf>
///
/// It's included in the US National Security Agency's "Suite B" and is widely
/// used in protocols like TLS and the associated X.509 PKI.
///
/// Its equation is `y² = x³ - 3x + b` over a ~384-bit prime field where `b` is
/// the "verifiably random"† constant:
///
/// ```text
/// b = 2758019355995970587784901184038904809305690585636156852142
///     8707301988689241309860865136260764883745107765439761230575
/// ```
///
/// † *NOTE: the specific origins of this constant have never been fully disclosed
///   (it is the SHA-1 digest of an inexplicable NSA-selected constant)*
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct NistP384;

impl elliptic_curve::Curve for NistP384 {
    /// 384-bit (48-byte)
    type ElementSize = U48;
}

#[cfg(feature = "oid")]
impl elliptic_curve::Identifier for NistP384 {
    const OID: ObjectIdentifier = ObjectIdentifier::new(&[1, 3, 132, 0, 34]);
}

impl elliptic_curve::weierstrass::Curve for NistP384 {
    const COMPRESS_POINTS: bool = false;
}

/// NIST P-384 Serialized Field Element.
///
/// Byte array containing a serialized field element value (base field or scalar).
pub type ElementBytes = elliptic_curve::ElementBytes<NistP384>;

/// NIST P-384 SEC1 Encoded Point.
pub type EncodedPoint = elliptic_curve::sec1::EncodedPoint<NistP384>;

/// NIST P-384 Secret Key
pub type SecretKey = elliptic_curve::SecretKey<NistP384>;
