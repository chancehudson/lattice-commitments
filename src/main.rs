use ring_math::polynomial_ring;
use ring_math::Polynomial;
use ring_math::PolynomialRingElement;
use ring_math::Vector;
use scalarff::custom_ring;
use scalarff::FieldElement;
use scalarff::FoiFieldElement;

mod commitment;

use commitment::Vcs;

// A small base field for testing
custom_ring!(F101, 101, "101_field");

/// Customization settings
const RING_DEGREE: usize = 64;
// Change this to FoiFieldElement to use 2^64-2^32+1 base field
type ActiveField = F101;

// TODO: adjust the cardinality of this ring
custom_ring!(BetaRing, 100, "beta_bound_ring");

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
    let x = Vector::<FieldPolynomial>::rand_uniform(vcs.l, &mut rand::thread_rng());
    // the short integer polynomial
    //
    // we calculate this here because of rust type restrictions in the current
    // implementation
    let r = Vector::from_vec(
        vec![FieldPolynomial::zero(); vcs.k]
            .iter()
            .map(|_| rand_beta())
            .collect::<Vec<_>>(),
    );

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
    let (alpha, commitment) = vcs.commit(&x, &r);
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

    println!("\nGenerating ZK proof of opening");
    let (t, z, d) = vcs.prove_opening(alpha.clone(), r);
    println!("ZK proof:");
    println!("t: {} polynomials", t.len());
    println!("z: {} polynomials", z.len());
    println!("d: {d}");
    let valid = vcs.verify_opening_proof(t, d, z, commitment, alpha);
    assert!(valid);
}

fn rand_beta() -> FieldPolynomial {
    let mut coefficients = vec![];
    for _ in 0..(FieldPolynomial::modulus().degree() - 1) {
        coefficients.push(ActiveField::from_biguint(
            &BetaRing::sample_rand(&mut rand::thread_rng()).to_biguint(),
        ));
    }
    FieldPolynomial::from(Polynomial { coefficients })
}
