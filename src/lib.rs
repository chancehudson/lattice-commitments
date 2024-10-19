//! lattice-commitments [![Build](https://img.shields.io/circleci/build/github/chancehudson/lattice-commitments/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/lattice-commitments/tree/main) [![Docs](https://img.shields.io/docsrs/lattice-commitments)](https://docs.rs/lattice-commitments) [![Version](https://img.shields.io/crates/v/lattice-commitments)](https://crates.io/crates/lattice-commitments)
//!
//! Structured lattice commitments based on [Baum et al.](https://eprint.iacr.org/2016/997.pdf)
use ring_math::polynomial_ring;
use ring_math::Polynomial;
use ring_math::PolynomialRingElement;
use scalarff::scalar_ring;
use scalarff::FieldElement;

pub mod commitment;

pub use commitment::Vcs;

scalar_ring!(BabyBearRingElement, 2013265921, "baby bear 32 bit");

const RING_DEGREE: usize = 1024;

polynomial_ring!(
    FieldPolynomial,
    BabyBearRingElement,
    {
        let mut p = Polynomial::identity();
        p.term(&BabyBearRingElement::one(), RING_DEGREE);
        p
    },
    "% x^RING_DEGREE + 1"
);
