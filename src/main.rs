use scalarff::custom_ring;
use scalarff::FieldElement;
use scalarff::FoiFieldElement;

mod commitment;
mod matrix;
mod norms;
mod polynomial;
mod ring_polynomial;

use commitment::Vcs;
use polynomial::Polynomial;

use ring_polynomial::RingPolynomial;

// A small base field for testing
custom_ring!(F101, 101, "101_field");

/// Customization settings
const RING_DEGREE: usize = 16;
// Change this to FoiFieldElement to use 2^64-2^32+1 base field
type ActiveField = F101;

// leave this as is
type FieldPolynomial = RingPolynomial<ActiveField>;

// TODO: adjust the cardinality of this ring
custom_ring!(BetaRing, 100, "beta_bound_ring");

fn main() {
    println!("Base field cardinality: {}", ActiveField::prime());
    println!(
        "Polynomial ring: ℤ[X]/<X^{} + 1>\n",
        RingPolynomial::<ActiveField>::degree()
    );
    let vcs = Vcs::new();
    // the value being committed to
    let x: Vec<FieldPolynomial> = vec![FieldPolynomial::zero(); vcs.l]
        .iter()
        .map(|_| FieldPolynomial::sample_rand(&mut rand::thread_rng()))
        .collect::<Vec<_>>();
    // the short integer polynomial
    //
    // we calculate this here because of rust type restrictions in the current
    // implementation
    let r: Vec<FieldPolynomial> = vec![FieldPolynomial::zero(); vcs.k]
        .iter()
        .map(|_| rand_beta())
        .collect::<Vec<_>>();

    // r and x are secret until opening
    // alpha is a public parameter
    println!(
        "Committing to {} polynomials, each containing {} coefficients:\n{}\n",
        vcs.l,
        RingPolynomial::<ActiveField>::degree(),
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
            commitment.len() * FieldPolynomial::byte_len()
        );
        println!(
            "Public parameters size: {} bytes",
            alpha_len * ActiveField::byte_len()
        );
        println!("Secret size: {} bytes", r.len() * ActiveField::byte_len());
    } else {
        println!("Commitment opening is NOT valid!")
    }
    // let (t, z, d) = vcs.prove(r, alpha.clone());
    // let valid = vcs.verify(t, z, d, commitment, alpha);

    assert!(valid);
}

fn rand_beta() -> FieldPolynomial {
    let mut coefficients = vec![];
    for _ in 0..(RingPolynomial::<ActiveField>::degree() - 1) {
        coefficients.push(ActiveField::from_biguint(
            &BetaRing::sample_rand(&mut rand::thread_rng()).to_biguint(),
        ));
    }
    RingPolynomial::from(Polynomial { coefficients })
}
