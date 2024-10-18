use ring_math::polynomial_ring;
use ring_math::Polynomial;
use ring_math::PolynomialRingElement;
use ring_math::Vector;
use scalarff::scalar_ring;
use scalarff::FieldElement;

mod commitment;

use commitment::Vcs;

// creates a scalar ring struct DilithiumRingElement
scalar_ring!(DilithiumRingElement, 8380417, "dilithium_23_bit");
scalar_ring!(BabyBearRingElement, 2013265921, "baby bear 32 bit");
scalar_ring!(F101, 101u128, "101 field");

// // Change this to OxfoiFieldElement to use 2^64-2^32+1 base field
type ActiveField = BabyBearRingElement;
const RING_DEGREE: usize = 1024;

polynomial_ring!(
    FieldPolynomial,
    ActiveField,
    {
        let mut p = Polynomial::identity();
        p.term(&ActiveField::one(), RING_DEGREE);
        p
    },
    "% x^RING_DEGREE + 1"
);

fn main() {
    println!("Base field cardinality: {}", ActiveField::prime());
    println!(
        "Polynomial ring: ℤ[X]/<X^{} + 1>\n",
        FieldPolynomial::modulus().degree()
    );
    let vcs = Vcs::new(RING_DEGREE);
    // the value being committed to
    let x = Vector::<FieldPolynomial>::sample_uniform(vcs.l, &mut rand::thread_rng());

    // r and x are secret until opening
    // alpha is a public parameter
    println!(
        "Committing to {} polynomials, each containing {} coefficients:\n{}\n",
        vcs.l,
        FieldPolynomial::modulus().degree(),
        x.iter()
            .map(|v| v.serialize())
            .collect::<Vec<_>>()
            .join(",\n")
    );
    let (alpha, commitment, r) = vcs.commit(&x, &mut rand::thread_rng());
    println!(
        "Opening commitment with secret vector ({} polynomials):\n{}\n",
        vcs.k,
        r.iter()
            .map(|v| v.serialize())
            .collect::<Vec<_>>()
            .join(",\n")
    );
    let valid = vcs.open(&commitment, &alpha, &x, &r);
    if valid {
        println!("Commitment opening is valid!\n");
        let alpha_len = alpha.dimensions.0 * alpha.dimensions.1;
        println!(
            "Commitment size: {} bytes",
            commitment.len() * RING_DEGREE * FieldPolynomial::byte_len()
        );
        println!(
            "Public parameters size: {} bytes",
            alpha_len * RING_DEGREE * ActiveField::byte_len()
        );
        println!(
            "Secret size: {} bytes",
            r.len() * RING_DEGREE * ActiveField::byte_len()
        );
    } else {
        println!("Commitment opening is NOT valid!")
    }

    assert!(valid);

    // TODO: fix zk proofs of opening
    //
    // println!("\nGenerating ZK proof of opening");
    // let (t, z, d) = vcs.prove_opening(alpha.clone(), r);
    // println!("ZK proof:");
    // println!("t: {} polynomials", t.len());
    // println!("z: {} polynomials", z.len());
    // println!("d: {d}");
    // let valid = vcs.verify_opening_proof(t, d, z, commitment, alpha);
    // assert!(valid);
}
