use scalarff::BigUint;
use scalarff::FieldElement;

use super::polynomial::Polynomial;

/// A concrete implementation of the polynomial ring
/// defined as `R_q[X]/<X^64 + 1>`
/// where q is the prime defined by `T::prime()`
///
/// This polynomial ring can be considered a field as long as it's
/// irreducible over the chosen base field T
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct RingPolynomial64<T: FieldElement>(Polynomial<T>);
const DEGREE: usize = 64;

impl<T: FieldElement> RingPolynomial64<T> {
    pub fn modulus() -> Self {
        let mut p = Polynomial::identity();
        p.term(&T::one(), 64);
        RingPolynomial64(p)
    }
}

impl<T: FieldElement> From<Polynomial<T>> for RingPolynomial64<T> {
    fn from(p: Polynomial<T>) -> Self {
        RingPolynomial64(p.div(&Self::modulus().0).1)
    }
}

impl<T: FieldElement> FieldElement for RingPolynomial64<T> {
    fn zero() -> Self {
        RingPolynomial64(Polynomial {
            coefficients: vec![T::zero()],
        })
    }

    fn one() -> Self {
        RingPolynomial64(Polynomial {
            coefficients: vec![T::one()],
        })
    }

    fn serialize(&self) -> String {
        panic!();
    }

    fn deserialize(str: &str) -> Self {
        panic!();
    }

    fn prime() -> BigUint {
        // cannot retrieve a scalar prime for a
        // polynomial field
        panic!();
    }

    fn name_str() -> &'static str {
        "polynomial_ring_64"
    }

    /// Return a constant polynomial with the provided
    /// value
    fn from_usize(value: usize) -> Self {
        RingPolynomial64(Polynomial {
            coefficients: vec![T::from_usize(value)],
        })
    }

    fn to_biguint(&self) -> BigUint {
        panic!();
    }

    fn from_biguint(v: &BigUint) -> Self {
        panic!();
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        panic!();
    }

    fn to_bytes_le(&self) -> Vec<u8> {
        panic!();
    }
}

impl<T: FieldElement> std::fmt::Display for RingPolynomial64<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: FieldElement> std::str::FromStr for RingPolynomial64<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

impl<T: FieldElement> From<u64> for RingPolynomial64<T> {
    fn from(value: u64) -> Self {
        RingPolynomial64::from(Polynomial {
            coefficients: vec![T::from(value)],
        })
    }
}

impl<T: FieldElement> std::ops::Add for RingPolynomial64<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RingPolynomial64::from(self.0 + other.0)
    }
}

impl<T: FieldElement> std::ops::Sub for RingPolynomial64<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        RingPolynomial64::from(self.0 - other.0)
    }
}

impl<T: FieldElement> std::ops::Mul for RingPolynomial64<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        RingPolynomial64::from(self.0 * other.0)
    }
}

impl<T: FieldElement> std::ops::Div for RingPolynomial64<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // this implementation implies floored division, so discard the remainder
        RingPolynomial64::from(self.0.div(&other.0).0)
    }
}

impl<T: FieldElement> std::ops::AddAssign for RingPolynomial64<T> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T: FieldElement> std::ops::MulAssign for RingPolynomial64<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T: FieldElement> std::ops::SubAssign for RingPolynomial64<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<T: FieldElement> std::ops::Neg for RingPolynomial64<T> {
    type Output = Self;

    fn neg(self) -> Self {
        RingPolynomial64(-self.0.clone())
    }
}
