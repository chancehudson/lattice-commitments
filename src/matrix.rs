use scalarff::FieldElement;

use super::ring64::RingPolynomial64;

/// A two dimensional matrix implementation
pub struct Matrix2D<T: FieldElement> {
    dimensions: (usize, usize), // (rows, cols)
    values: Vec<RingPolynomial64<T>>,
}

impl<T: FieldElement> Matrix2D<T> {}
