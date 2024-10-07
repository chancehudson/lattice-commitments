use std::marker::PhantomData;

use scalarff::FieldElement;

use super::matrix::Matrix2D;
use super::norms::Norm;
use super::vector::Vector;

#[derive(Clone, Debug)]
pub struct Vcs<T: FieldElement + Norm> {
    _phantom: PhantomData<T>,
    pub k: usize,     // Width (over Rq) of the commitment matrices
    pub n: usize,     // Height (over Rq) of the commitment matrix A_1
    pub l: usize,     // Dimension (over Rq) of the message space
    pub theta: usize, // Dimension (over Rq) of the randomness space
}

impl<T: FieldElement + Norm> Vcs<T> {
    pub fn new() -> Self {
        // requirements
        // n < k
        // k > n + l
        Vcs {
            _phantom: PhantomData,
            k: 8,
            n: 2,
            l: 2,
            theta: 33792000,
        }
    }

    /// Commit to a value `x` with secret `r`
    ///
    /// Returns the public parameters alpha and the commitment vector
    pub fn commit(&self, x: &Vector<T>, r: &Vector<T>) -> (Matrix2D<T>, Vector<T>) {
        assert_eq!(self.l, x.len(), "invalid message length");
        let (alpha_1, alpha_2) = self.public_params();
        let alpha = alpha_1.compose_vertical(alpha_2.clone());
        println!("alpha matrix: {}", alpha);

        // matrix vector multiplication
        let inter1 = alpha.clone() * r.clone();
        let inter2 = Vector::from_vec([vec![T::zero(); self.n], x.to_vec()].concat());
        let commitment = inter2.clone() + inter1.clone();

        (alpha, commitment)
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
            if v.norm_l2() > u64::try_from(4 * self.theta * 8).unwrap() {
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
            Matrix2D::rand_uniform(self.n, self.k - self.n, &mut rand::thread_rng());
        let alpha_2_prime =
            Matrix2D::rand_uniform(self.l, self.k - self.n - self.l, &mut rand::thread_rng());
        let alpha_1 = Matrix2D::identity(self.n).compose_horizontal(alpha_1_prime);
        let alpha_2 = Matrix2D::zero(self.l, self.n)
            .compose_horizontal(Matrix2D::identity(self.l))
            .compose_horizontal(alpha_2_prime);
        (alpha_1, alpha_2)
    }
}
