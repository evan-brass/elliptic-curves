//! ECDSA verifier

use super::{recoverable, Error, Signature};
use crate::{
    AffinePoint, ElementBytes, EncodedPoint, NonZeroScalar, ProjectivePoint, Scalar, Secp256k1,
};
use ecdsa_core::{hazmat::VerifyPrimitive, signature};
use elliptic_curve::{ops::Invert, FromBytes};

/// ECDSA/secp256k1 verifier
#[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
pub struct Verifier {
    /// Core ECDSA verifier
    verifier: ecdsa_core::Verifier<Secp256k1>,
}

impl Verifier {
    /// Create a new verifier
    pub fn new(public_key: &EncodedPoint) -> Result<Self, Error> {
        Ok(Self {
            verifier: ecdsa_core::Verifier::new(public_key)?,
        })
    }
}

impl signature::Verifier<Signature> for Verifier {
    fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), Error> {
        self.verifier.verify(msg, signature)
    }
}

impl signature::Verifier<recoverable::Signature> for Verifier {
    fn verify(&self, msg: &[u8], signature: &recoverable::Signature) -> Result<(), Error> {
        self.verifier.verify(msg, &Signature::from(*signature))
    }
}

impl VerifyPrimitive<Secp256k1> for AffinePoint {
    fn verify_prehashed(
        &self,
        hashed_msg: &ElementBytes,
        signature: &Signature,
    ) -> Result<(), Error> {
        let maybe_r = NonZeroScalar::from_bytes(signature.r());
        let maybe_s = NonZeroScalar::from_bytes(signature.s());

        // TODO(tarcieri): replace with into conversion when available (see subtle#73)
        let (r, s) = if maybe_r.is_some().into() && maybe_s.is_some().into() {
            (maybe_r.unwrap(), maybe_s.unwrap())
        } else {
            return Err(Error::new());
        };

        // Ensure signature is "low S" normalized ala BIP 0062
        if s.as_ref().is_high().into() {
            return Err(Error::new());
        }

        let z = Scalar::from_bytes_reduced(&hashed_msg);
        let s_inv = s.invert().unwrap();
        let u1 = z * &s_inv;
        let u2 = r.as_ref() * &s_inv;

        let x = ((&ProjectivePoint::generator() * &u1) + &(ProjectivePoint::from(*self) * &u2))
            .to_affine()
            .unwrap()
            .x;

        if Scalar::from_bytes_reduced(&x.to_bytes()).eq(r.as_ref()) {
            Ok(())
        } else {
            Err(Error::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_vectors::ecdsa::ECDSA_TEST_VECTORS, AffinePoint};
    use ecdsa_core::generic_array::GenericArray;

    ecdsa_core::new_verification_test!(ECDSA_TEST_VECTORS);
}
