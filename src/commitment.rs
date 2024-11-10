use std::collections::HashMap;
use std::marker::PhantomData;

use anyhow::anyhow;
use anyhow::Result;
use keccak_hash::keccak256;
use rand::Rng;
use ring_math::Matrix2D;
use ring_math::Polynomial;
use ring_math::PolynomialRingElement;
use ring_math::Vector;
use scalarff::scalar_ring;
use scalarff::BigUint;
use scalarff::FieldElement;

use crate::ChaChaRng;

/// Instance of a vector commitment scheme. Contains functions
/// for committing to a vector and verifying the commitment.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Vcs<T: PolynomialRingElement> {
    _phantom: PhantomData<T>,
    pub k: usize,     // Width (over Rq) of the commitment matrices
    pub n: usize,     // Height (over Rq) of the commitment matrix A_1
    pub l: usize,     // Dimension (over Rq) of the message space
    pub beta: usize,  // infinite norm bound for prover randomness vector
    pub kappa: usize, // l1 norm bound for challenge vectors
    pub theta: f64,   // standard deviation in discrete gaussian distribution
    pub N: usize,     // degree of the ring modulus
}

fn f64_to_u64(v: f64) -> u64 {
    let z = v.ceil() as u64;
    z
}

scalar_ring!(BetaRing, 2, "beta_bound_ring");

impl<T: PolynomialRingElement + serde::Serialize> Vcs<T> {
    /// Construct a new vector commitment scheme instance.
    pub fn new(polynomial_degree: usize) -> Self {
        // requirements
        // n < k
        // k > n + l
        let kappa: u32 = 36;
        let beta: u32 = 1;
        let k: u32 = 3;
        let n: u32 = 1;
        let l: u32 = 1;
        if kappa > polynomial_degree as u32 {
            panic!("kappa must be less than the polynomial degree otherwise challenge vector does not exist");
        }
        Vcs {
            N: polynomial_degree,
            _phantom: PhantomData,
            k: usize::try_from(k).unwrap(),
            n: usize::try_from(n).unwrap(),
            l: usize::try_from(l).unwrap(),
            beta: usize::try_from(beta).unwrap(),
            kappa: usize::try_from(kappa).unwrap(),
            // theta: 33792000,
            theta: 11.0 * f64::from(kappa * beta) * f64::sqrt(f64::from(k) * 1024.),
        }
    }

    /// Sample a challenge vector with a specified l_1 and l_infinite norm
    ///
    /// l_inf should be 1 and l1 should be kappa
    pub fn sample_challenge_vector(&self, t: &Vector<T>) -> Result<T> {
        // generate random values in range 0..N
        // if duplicate value returned discard?
        let bytes = bincode::serialize(t)?;
        let mut rng = ChaChaRng::from_seed(&bytes);

        let mut existing: HashMap<usize, bool> = HashMap::new();
        while existing.len() < self.kappa {
            let degree = rng.gen_range(0..self.N);
            if existing.contains_key(&degree) {
                continue;
            }
            existing.insert(degree, true);
        }
        let mut poly = T::zero().polynomial().clone();
        for (degree, _) in existing.iter() {
            poly.term(&T::F::one(), *degree);
        }
        let out = T::from(poly);
        if out.norm_l1() != BigUint::from(u64::try_from(self.kappa).unwrap()) {
            anyhow::bail!("Challenge vector does not have the correct l1 norm");
        }
        if out.norm_max() != BigUint::from(1u64) {
            anyhow::bail!("Challenge vector does not have the correct l max norm");
        }
        Ok(out)
    }

    /// Generate a proof that the user knows the opening value of a commitment.
    ///
    /// Similar to proving knowledge of a hash pre-image.
    #[cfg(feature = "zk")]
    pub fn prove_opening(
        &self,
        alpha: Matrix2D<T>,
        r: Vector<T>,
    ) -> Result<(Vector<T>, Vector<T>)> {
        // sample a y using a discrete gaussian distribution
        let y = Vector::from_vec(
            vec![T::zero(); self.k]
                .iter()
                .map(|_| {
                    T::from(u64::from(discrete_gaussian::sample_vartime(
                        self.theta,
                        &mut rand::thread_rng(),
                    )))
                })
                .collect::<Vec<_>>(),
        );
        let (alpha_1, _alpha_2) = self.decompose_alpha(alpha);
        let t = alpha_1.clone() * y.clone();

        // challenge vector sampled from verifier randomness
        let d = self.sample_challenge_vector(&t)?;
        println!("{d}");
        println!("{r}");
        println!("afhkajsf");
        assert_eq!((r[0].clone() * d.clone()) / d.clone(), r[0].clone());
        println!("{}", (r[0].clone() * d.clone()).norm_l2());
        panic!("");

        let z = y + r * d.clone();

        Ok((t, z))
    }

    /// Verify a proof that a user knows the opening value of a commitment.
    #[cfg(feature = "zk")]
    pub fn verify_opening_proof(
        &self,
        t: Vector<T>,
        z: Vector<T>,
        cm: Vector<T>,
        alpha: Matrix2D<T>,
    ) -> Result<bool> {
        // check that the l2_norm for each element in z is <= 2 * theta * sqrt(N)
        // check that A_1 * z = t + d * c_1

        for v in z.iter() {
            println!("{}", v.norm_l2());
            if v.norm_l2() > BigUint::from(f64_to_u64(2.0 * self.theta * 8.0)) {
                return Ok(false);
            }
        }
        let d = self.sample_challenge_vector(&t)?;

        let (alpha_1, _alpha_2) = self.decompose_alpha(alpha);
        let (cm_1, _cm_2) = self.decompose_cm(cm);
        let lhs = alpha_1 * z;
        let rhs = t + cm_1 * d;

        Ok(lhs == rhs)
    }

    /// Commit to a value `x`
    ///
    /// Returns the public parameters alpha, the commitment vector, and the secret r
    pub fn commit<R: rand::Rng>(
        &self,
        x: &Vector<T>,
        rng: &mut R,
    ) -> (Matrix2D<T>, Vector<T>, Vector<T>) {
        assert_eq!(self.l, x.len(), "invalid message length");

        // the short integer polynomial
        let r = Vector::from_vec(
            vec![T::zero(); self.k]
                .iter()
                .map(|_| Self::sample_beta(rng))
                .collect::<Vec<_>>(),
        );

        #[cfg(debug_assertions)]
        {
            // check the l_inf norm of the r
            for v in r.iter() {
                assert!(v.norm_max() < BetaRing::prime());
                assert!(v.norm_l1() != BigUint::from(0u32));
            }
        }

        let (alpha_1, alpha_2) = self.public_params();
        use std::time::Instant;
        let now = Instant::now();
        let alpha = alpha_1.compose_vertical(alpha_2.clone());

        // matrix vector multiplication
        let inter1 = alpha.clone() * r.clone();
        let inter2 = Vector::from_vec([vec![T::zero(); self.n], x.to_vec()].concat());
        let commitment = inter2.clone() + inter1.clone();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        (alpha, commitment, r)
    }

    /// Open a previously created commitment
    /// commitment: the commitment vector being verified
    /// alpha_1: random public parameter selected during commitment
    /// x: the message that was committed to
    /// r: a polynomial with l2 norm less than 4*theta*sqrt(N)
    ///
    /// Solving for r without previous knowledge should involve solving
    /// the modular SIS problem (hard).
    pub fn open(
        &self,
        commitment: &Vector<T>,
        alpha: &Matrix2D<T>,
        x: &Vector<T>,
        r: &Vector<T>,
    ) -> bool {
        for v in r.iter() {
            if v.norm_l2() > BigUint::from(f64_to_u64(4.0 * self.theta * 8.0)) {
                return false;
            }
        }
        let padded_x = Vector::from_vec([vec![T::zero(); self.n], x.to_vec()].concat());
        let c = alpha.clone() * r.clone() + padded_x.clone();
        c == *commitment
    }

    /// Generate random public params for use in the scheme
    pub fn public_params(&self) -> (Matrix2D<T>, Matrix2D<T>) {
        let alpha_1_prime =
            Matrix2D::sample_uniform(self.n, self.k - self.n, &mut rand::thread_rng());
        let alpha_2_prime =
            Matrix2D::sample_uniform(self.l, self.k - self.n - self.l, &mut rand::thread_rng());
        let alpha_1 = Matrix2D::identity(self.n).compose_horizontal(alpha_1_prime);
        let alpha_2 = Matrix2D::zero(self.l, self.n)
            .compose_horizontal(Matrix2D::identity(self.l))
            .compose_horizontal(alpha_2_prime);
        (alpha_1, alpha_2)
    }

    /// Decompose an alpha matrix into A_1 and A_2
    pub fn decompose_alpha(&self, alpha: Matrix2D<T>) -> (Matrix2D<T>, Matrix2D<T>) {
        alpha.split_vertical(self.n, self.l)
    }

    /// Decompose a commitment to c_1 and c_2
    pub fn decompose_cm(&self, cm: Vector<T>) -> (Vector<T>, Vector<T>) {
        let v = cm.0.clone();
        (Vector(v[..self.n].to_vec()), Vector(v[self.n..].to_vec()))
    }

    /// Sample an element in S_β
    fn sample_beta<R: rand::Rng>(rng: &mut R) -> T {
        // maybe put this sampling in the polynomial implementation?
        T::from_polynomial(Polynomial {
            coefficients: T::zero()
                .coef()
                .iter()
                .map(|_| BetaRing::sample_uniform(rng).to_biguint())
                .map(|v| T::F::from_biguint(&v))
                .collect::<Vec<_>>(),
        })
    }

    /// infinite norm bound for generating S_β elements
    pub fn beta_bound() -> BigUint {
        BetaRing::prime()
    }
}
