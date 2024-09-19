use std::marker::PhantomData;

use rand::Rng;
use scalarff::FieldElement;

use super::matrix::Matrix2D;
use super::norms::Norm;

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
    pub fn commit(&self, x: &Vec<T>, r: &Vec<T>) -> (Matrix2D<T>, Vec<T>) {
        assert_eq!(self.l, x.len(), "invalid message length");
        let (alpha_1, alpha_2) = self.public_params();
        let alpha = alpha_1.compose_vertical(alpha_2.clone());

        let inter1 = Self::vec_matrix_mul(r.clone(), alpha.clone());
        let inter2 = vec![vec![T::zero(); self.n], x.clone()].concat();
        let commitment = Self::vec_add(&inter2, &inter1);

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
    pub fn open(&self, commitment: &Vec<T>, alpha: &Matrix2D<T>, x: &Vec<T>, r: &Vec<T>) -> bool {
        for v in r {
            if v.norm_l2() > u64::try_from(4 * self.theta * 8).unwrap() {
                return false;
            }
        }
        let mut padded_x = vec![T::zero(); self.n];
        padded_x.append(&mut x.clone());
        let c = Self::vec_add(&Self::vec_matrix_mul(r.clone(), alpha.clone()), &padded_x);
        c == *commitment
    }

    pub fn prove(&self, r: Vec<T>, alpha: Matrix2D<T>) -> (Vec<T>, Vec<T>, T) {
        let y = self.rand_theta_vec(self.k);
        let (alpha_1, alpha_2) = alpha.split_vertical(self.n, self.l);
        let t = Self::vec_matrix_mul(y.clone(), alpha_1.clone());
        // TODO: determine a d value based on t
        let d = T::sample_rand(&mut rand::thread_rng());
        // TODO: abort if necessary
        let z = Self::vec_add(&y, &r.iter().map(|v| v.clone() * d.clone()).collect());
        (t, z, d)
    }

    pub fn verify(
        &self,
        t: Vec<T>,
        z: Vec<T>,
        d: T,
        commitment: Vec<T>,
        alpha: Matrix2D<T>,
    ) -> bool {
        // accept if alpha_1 * z = t + d * c1
        let (alpha_1, alpha_2) = alpha.split_vertical(self.n, self.l);
        let lhs = Self::vec_matrix_mul(z.clone(), alpha_1.clone());
        let c1 = commitment[0..self.n].to_vec();
        let rhs = Self::vec_add(&t, &Self::scalar_vec_mul(d, &c1));
        for i in 0..lhs.len() {
            if lhs[i] != rhs[i] {
                return false;
            }
        }
        // and all z l2_norms are < 2*theta*N^(1/2)
        // 2 * theta * N^(1/2) = 67584000
        let m = 67584000_u64;
        for v in z {
            println!("{}", v.norm_l2());
            if v.norm_l2() > m {
                return false;
            }
        }
        true
    }

    /// Generate random public params for use in the scheme
    pub fn public_params(&self) -> (Matrix2D<T>, Matrix2D<T>) {
        let alpha_1_prime = Self::rand_matrix(self.n, self.k - self.n);
        let alpha_2_prime = Self::rand_matrix(self.l, self.k - self.n - self.l);
        let alpha_1 = Matrix2D::identity(self.n).compose_horizontal(alpha_1_prime);
        let alpha_2 = Matrix2D::zero(self.l, self.n)
            .compose_horizontal(Matrix2D::identity(self.l))
            .compose_horizontal(alpha_2_prime);
        (alpha_1, alpha_2)
    }

    fn rand_theta_vec(&self, n: usize) -> Vec<T> {
        vec![0; n]
            .into_iter()
            .map(|_| rand::thread_rng().gen_range(0..self.theta))
            .map(|v| T::from_usize(v) - T::from_usize(self.theta / 2))
            .collect()
    }

    fn rand_vec(n: usize) -> Vec<T> {
        vec![0; n]
            .into_iter()
            .map(|_| T::sample_rand(&mut rand::thread_rng()))
            .collect()
    }

    fn rand_matrix(r: usize, c: usize) -> Matrix2D<T> {
        Matrix2D {
            dimensions: (r, c),
            values: Self::rand_vec(r * c),
        }
    }

    fn vec_matrix_mul(v: Vec<T>, m: Matrix2D<T>) -> Vec<T> {
        let mut out = Vec::new();
        let (m_rows, m_cols) = m.dimensions;
        for i in 0..m_rows {
            let row = m.values[i * m_cols..(i + 1) * m_cols].to_vec();

            out.push(
                // TODO: determine if summing the vector here is correct
                Self::vec_mul(&v, &row)
                    .iter()
                    .fold(T::zero(), |acc, v| acc + v.clone()),
            );
        }
        out
    }

    fn vec_mul(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
        assert_eq!(v1.len(), v2.len(), "vector mul length mismatch");
        let mut out = Vec::new();
        for i in 0..v1.len() {
            out.push(v1[i].clone() * v2[i].clone());
        }
        out
    }

    // this addition happens collumn wise 🙄
    fn vec_matrix_add(v: Vec<T>, m: Matrix2D<T>) -> Matrix2D<T> {
        let mut out = Vec::new();
        let (m_rows, m_cols) = m.dimensions;
        for i in 0..m_rows {
            let row = m.values[i * m_cols..(i + 1) * m_cols].to_vec();
            out.append(
                &mut row
                    .iter()
                    .map(|z| z.clone() * v[i].clone())
                    .collect::<Vec<T>>(),
            );
        }
        Matrix2D {
            dimensions: m.dimensions,
            values: out,
        }
    }

    fn vec_add(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
        assert_eq!(v1.len(), v2.len(), "vector add length mismatch");
        let mut out = Vec::new();
        for i in 0..v1.len() {
            out.push(v1[i].clone() + v2[i].clone());
        }
        out
    }

    fn scalar_vec_mul(s: T, v: &Vec<T>) -> Vec<T> {
        let mut out = Vec::new();
        for r in v {
            out.push(r.clone() * s.clone());
        }
        out
    }

    fn scalar_matrix_mul(s: T, m: &Matrix2D<T>) -> Matrix2D<T> {
        Matrix2D {
            dimensions: m.dimensions,
            values: m.values.iter().map(|v| v.clone() * s.clone()).collect(),
        }
    }

    fn print_matrix(m: &Matrix2D<T>) {
        let (m_rows, m_cols) = m.dimensions;
        for i in 0..m_rows {
            let r = m.values[i * m_cols..(i + 1) * m_cols].to_vec();
            for v in r {
                print!("{}, ", v.serialize());
            }
            println!("");
        }
    }
}
