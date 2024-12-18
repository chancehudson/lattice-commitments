use commitment::BetaRing;
use ring_math::polynomial_ring;
use ring_math::Polynomial;
use ring_math::PolynomialRingElement;
use ring_math::Vector;
use scalarff::scalar_ring;
use scalarff::FieldElement;

mod commitment;

use commitment::Vcs;

// creates a scalar ring struct DilithiumRingElement
scalar_ring!(DilithiumRingElement, 8380417, "dilithium 23 bit");
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
    println!(
        "Base field cardinality: {} ({})",
        ActiveField::prime(),
        ActiveField::name_str()
    );
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
        {
            let mut s = x
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",\n");
            s.truncate(256);
            s += "...";
            s
        }
    );
    let (alpha, commitment, r) = vcs.commit(&x, &mut rand::thread_rng());
    println!(
        "Opening commitment with secret vector ({} polynomials):\n{}\n",
        vcs.k,
        {
            let mut s = r
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",\n");
            s.truncate(256);
            s += "...";
            s
        }
    );
    let valid = vcs.open(&commitment, &alpha, &x, &r);
    if valid {
        println!("Commitment opening is valid!\n");
        println!(
            "Commitment size: {} bytes",
            commitment.len() * FieldPolynomial::byte_len()
        );
        println!(
            "Public parameters size: {} bytes",
            alpha.dimensions.0 * alpha.dimensions.1 * FieldPolynomial::byte_len()
        );
        println!("Secret size: {} bytes", r.len() * BetaRing::byte_len());
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
