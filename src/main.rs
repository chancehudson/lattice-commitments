use rand::prelude::*;
use scalarff::matrix::Matrix;
use scalarff::FieldElement;
use scalarff::FoiFieldElement;

mod matrix;
mod polynomial;
mod ring64;

use ring64::RingPolynomial64;

type FieldPolynomial = RingPolynomial64<FoiFieldElement>;

fn main() {
    let mut x: Vec<FieldPolynomial> = Vec::new();
    for _ in 0..16 {
        x.push(rand_field());
    }
    // r and x are secret until opening
    // alpha 1 and 2 are public parameters
    let (r, alpha, commitment) = commit(x);
    let (t, z, d) = prove(r, alpha.clone());
    let valid = verify(t, z, d, commitment, alpha);

    assert!(valid);
}

fn verify(
    t: Vec<FieldPolynomial>,
    z: Vec<FieldPolynomial>,
    d: FieldPolynomial,
    commitment: Vec<FieldPolynomial>,
    alpha: Matrix<FieldPolynomial>,
) -> bool {
    // requirements
    // n < k
    // k > n + l
    let n = 16;
    let k = 64;
    let l = 16;
    // accept if alpha_1 * z = t + d * c1
    let (alpha_1, alpha_2) = matrix_split_vertical(alpha, n, l);
    let lhs = vec_matrix_mul(z.clone(), alpha_1.clone());
    let c1 = commitment[0..n].to_vec();
    let rhs = vec_add(&t, &scalar_vec_mul(d, &c1));
    for i in 0..lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
    }
    // and all z l2_norms are < 2*theta*N^(1/2)
    // 2 * theta * N^(1/2) = 67584000
    // let m = 67584000_u128;
    // if l2_norm(z) > m {
    //     return false;
    // }
    true
}

fn prove(
    r: Vec<FieldPolynomial>,
    alpha: Matrix<FieldPolynomial>,
) -> (Vec<FieldPolynomial>, Vec<FieldPolynomial>, FieldPolynomial) {
    // requirements
    // n < k
    // k > n + l
    let n = 16;
    let k = 64;
    let l = 16;
    let y = rand_vec(k);
    let (alpha_1, alpha_2) = matrix_split_vertical(alpha, n, l);
    let t = vec_matrix_mul(y.clone(), alpha_1.clone());
    // TODO: determine a d value based on t
    let d = rand_field();
    // TODO: abort if necessary
    let z = vec_add(&y, &r.iter().map(|v| v * d).collect());
    (t, z, d)
}

// returns r vector and commitment matrix
fn commit(
    x: Vec<FieldPolynomial>,
) -> (
    Vec<FieldPolynomial>,
    Matrix<FieldPolynomial>,
    Vec<FieldPolynomial>,
) {
    // requirements
    // n < k
    // k > n + l
    let n = 16;
    let k = 64;
    let l = 16; // message length
    assert_eq!(l, x.len(), "invalid message length");

    // let lambda = 256;
    // let kappa = 60;

    // alpha1 and 2 are public parameters in the system
    // described at the top of page 11
    let alpha_1_prime = rand_matrix(n, k - n);
    let alpha_2_prime = rand_matrix(n, k - n - l);

    let alpha_1 = compose_horizontal(identity_matrix(n), alpha_1_prime);
    let alpha_2 = compose_horizontal(
        compose_horizontal(zero_matrix(l, n), identity_matrix(l)),
        alpha_2_prime,
    );
    let alpha = compose_vertical(alpha_1.clone(), alpha_2.clone());

    // modinv of 64 in F = 2^64 - 2^58
    // beta must be less than modinv(64)

    let beta = 100_u64;
    let r = vec![0; k]
        .into_iter()
        .map(|_| rand_field() % beta)
        .map(|v| FieldPolynomial::from(v))
        .collect::<Vec<FieldPolynomial>>();

    let inter1 = vec_matrix_mul(r.clone(), alpha.clone());
    let inter2 = vec![vec![FieldPolynomial::zero(); n], x].concat();
    let commitment = vec_add(&inter2, &inter1);

    // print_matrix(&commitment);
    let (a1, a2) = matrix_split_vertical(alpha.clone(), n, l);
    assert!(a1 == alpha_1);
    assert!(a2 == alpha_2);

    (r, alpha, commitment)
}

fn rand_field() -> FieldPolynomial {
    FieldPolynomial::from(rand::random::<u64>())
}

fn rand_vec(n: usize) -> Vec<FieldPolynomial> {
    vec![0; n].into_iter().map(|_| rand_field()).collect()
}

fn identity_matrix(n: usize) -> Matrix<FieldPolynomial> {
    let mut values: Vec<FieldPolynomial> = Vec::new();
    for x in 0..n {
        let mut row = vec![FieldPolynomial::zero(); n];
        row[x] = FieldPolynomial::one();
        values.append(&mut row);
    }
    Matrix {
        dimensions: vec![n, n],
        values,
    }
}

fn zero_matrix(r: usize, c: usize) -> Matrix<FieldPolynomial> {
    Matrix {
        dimensions: vec![r, c],
        values: vec![FieldPolynomial::zero(); r * c],
    }
}

fn rand_matrix(r: usize, c: usize) -> Matrix<FieldPolynomial> {
    Matrix {
        dimensions: vec![r, c],
        values: rand_vec(r * c),
    }
}

/// Take an input matrix `m` and split it vertically into two
/// matrices of heights `m1_height` and `m2_height`.
/// The sum of `m1_height` and `m2_height` must be equal to the
/// height of `m`
fn matrix_split_vertical(
    m: Matrix<FieldPolynomial>,
    m1_height: usize,
    m2_height: usize,
) -> (Matrix<FieldPolynomial>, Matrix<FieldPolynomial>) {
    assert_eq!(
        m.len(),
        m1_height + m2_height,
        "matrix vertical split height mismatch"
    );
    let cols = m.dimensions[1];
    let mid_offset = m1_height * cols;
    (
        Matrix {
            dimensions: vec![m1_height, cols],
            values: m.values[..mid_offset].to_vec(),
        },
        Matrix {
            dimensions: vec![m2_height, cols],
            values: m.values[mid_offset..].to_vec(),
        },
    )
}

/// Join two matrices horizontally
fn compose_horizontal(
    m1: Matrix<FieldPolynomial>,
    m2: Matrix<FieldPolynomial>,
) -> Matrix<FieldPolynomial> {
    assert_eq!(
        m1.dimensions[0], m2.dimensions[0],
        "vertical size mismatch in horizontal composition"
    );
    let mut values = vec![];
    let m1_rows = m1.dimensions[0];
    let m1_cols = m1.dimensions[1];
    let m2_rows = m2.dimensions[0];
    let m2_cols = m2.dimensions[1];
    assert_eq!(
        m1_rows, m2_rows,
        "vertical size mismatch in horizontal composition"
    );
    for i in 0..m1_rows {
        values.append(&mut m1.values[i * m1_cols..(i + 1) * m1_cols].to_vec());
        values.append(&mut m1.values[i * m2_cols..(i + 1) * m2_cols].to_vec());
    }
    Matrix {
        dimensions: vec![m1.dimensions[0], m1.dimensions[1] + m2.dimensions[1]],
        values,
    }
}

/// Join two matrices vertically
fn compose_vertical(
    m1: Matrix<FieldPolynomial>,
    m2: Matrix<FieldPolynomial>,
) -> Matrix<FieldPolynomial> {
    let m1_rows = m1.dimensions[0];
    let m1_cols = m1.dimensions[1];
    let m2_rows = m2.dimensions[0];
    let m2_cols = m2.dimensions[1];
    assert_eq!(
        m1_cols, m2_cols,
        "horizontal size mismatch in vertical composition"
    );
    Matrix {
        dimensions: vec![m1_rows + m2_rows, m1_cols],
        values: m1.values.iter().chain(m2.values.iter()).cloned().collect(),
    }
}

fn vec_matrix_mul(v: Vec<FieldPolynomial>, m: Matrix<FieldPolynomial>) -> Vec<FieldPolynomial> {
    let mut out = Vec::new();
    let m_cols = m.dimensions[1];
    let m_rows = m.dimensions[0];
    for i in 0..m_rows {
        let row = m.values[i * m_cols..(i + 1) * m_cols].to_vec();
        out.push(l1_norm(vec_mul(&v, &row)));
    }
    out
}

fn vec_mul(v1: &Vec<FieldPolynomial>, v2: &Vec<FieldPolynomial>) -> Vec<FieldPolynomial> {
    assert_eq!(v1.len(), v2.len(), "vector mul length mismatch");
    let mut out = Vec::new();
    for i in 0..v1.len() {
        out.push(v1[i].clone() * v2[i].clone());
    }
    out
}

// this addition happens collumn wise 🙄
fn vec_matrix_add(v: Vec<FieldPolynomial>, m: Matrix<FieldPolynomial>) -> Matrix<FieldPolynomial> {
    let mut out = Vec::new();
    let m_cols = m.dimensions[1];
    let m_rows = m.dimensions[0];
    for i in 0..m_rows {
        let row = m.values[i * m_cols..(i + 1) * m_cols].to_vec();
        out.append(
            &mut row
                .iter()
                .map(|z| z.clone() * v[i].clone())
                .collect::<Vec<FieldPolynomial>>(),
        );
    }
    Matrix {
        dimensions: m.dimensions,
        values: out,
    }
}

fn vec_add(v1: &Vec<FieldPolynomial>, v2: &Vec<FieldPolynomial>) -> Vec<FieldPolynomial> {
    assert_eq!(v1.len(), v2.len(), "vector add length mismatch");
    let mut out = Vec::new();
    for i in 0..v1.len() {
        out.push(v1[i].clone() + v2[i].clone());
    }
    out
}

fn scalar_vec_mul(s: FieldPolynomial, v: &Vec<FieldPolynomial>) -> Vec<FieldPolynomial> {
    let mut out = Vec::new();
    for r in v {
        out.push(r.clone() * s.clone());
    }
    out
}

fn scalar_matrix_mul(s: FieldPolynomial, m: &Matrix<FieldPolynomial>) -> Matrix<FieldPolynomial> {
    Matrix {
        dimensions: m.dimensions.clone(),
        values: m.values.iter().map(|v| v.clone() * s.clone()).collect(),
    }
}

fn print_matrix(m: &Matrix<FieldPolynomial>) {
    let m_rows = m.dimensions[0];
    let m_cols = m.dimensions[1];
    for i in 0..m_rows {
        let r = m.values[i * m_cols..(i + 1) * m_cols].to_vec();
        for v in r {
            print!("{}, ", v.serialize());
        }
        println!("");
    }
}
