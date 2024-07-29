use ark_ff::PrimeField;
use rand::prelude::*;

use ark_ff::fields::{Fp64, MontBackend, MontConfig};
use ark_ff::Field;

#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "3"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

fn main() {
    let mut x: Vec<u128> = Vec::new();
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
    t: Vec<u128>,
    z: Vec<u128>,
    d: u128,
    commitment: Vec<u128>,
    alpha: Vec<Vec<u128>>,
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

fn prove(r: Vec<u128>, alpha: Vec<Vec<u128>>) -> (Vec<u128>, Vec<u128>, u128) {
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
    let z = vec_add(&y, &r.iter().map(|v| (v * d) % F).collect());
    (t, z, d)
}

// returns r vector and commitment matrix
fn commit(x: Vec<u128>) -> (Vec<u128>, Vec<Vec<u128>>, Vec<u128>) {
    // requirements
    // n < k
    // k > n + l
    let n = 16;
    let k = 64;
    let l = 16;
    assert_eq!(l, x.len(), "invalid message length");

    // let lambda = 256;
    // let kappa = 60;

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

    let beta = 100_u128;
    let r = vec![0; k]
        .into_iter()
        .map(|_| rand_field() % beta)
        .collect::<Vec<u128>>();

    let inter1 = vec_matrix_mul(r.clone(), alpha.clone());
    let inter2 = vec![vec![0; n], x].concat();
    let commitment = vec_add(&inter2, &inter1);

    // print_matrix(&commitment);
    let (a1, a2) = matrix_split_vertical(alpha.clone(), n, l);
    assert!(matrix_equal(&a1, &alpha_1));
    assert!(matrix_equal(&a2, &alpha_2));

    (r, alpha, commitment)
}

static F: u128 = 18446744069414584321;

fn rand_field() -> u128 {
    rand::random::<u128>() % F
}

fn rand_vec(n: usize) -> Vec<u128> {
    vec![0; n].into_iter().map(|_| rand_field()).collect()
}

fn identity_matrix(n: usize) -> Vec<Vec<u128>> {
    let mut out: Vec<Vec<u128>> = Vec::new();
    for x in 0..n {
        let mut row = vec![0_u128; n];
        row[x] = 1_u128;
        out.push(row)
    }
    out
}

fn zero_matrix(r: usize, c: usize) -> Vec<Vec<u128>> {
    vec![vec![0_u128; c]; r]
}

fn rand_matrix(r: usize, c: usize) -> Vec<Vec<u128>> {
    let mut out: Vec<Vec<u128>> = Vec::new();
    for _ in 0..r {
        out.push(rand_vec(c));
    }
    out
}

fn matrix_equal(m1: &Vec<Vec<u128>>, m2: &Vec<Vec<u128>>) -> bool {
    assert_eq!(m1.len(), m2.len(), "matrix equality row count mismatch");
    for x in 0..m1.len() {
        assert_eq!(
            m1[x].len(),
            m2[x].len(),
            "matrix equality row length mismatch"
        );
        for y in 0..m1[x].len() {
            if m1[x][y] != m2[x][y] {
                return false;
            }
        }
    }
    true
}

fn matrix_split_vertical(
    m: Vec<Vec<u128>>,
    m1_height: usize,
    m2_height: usize,
) -> (Vec<Vec<u128>>, Vec<Vec<u128>>) {
    assert_eq!(
        m.len(),
        m1_height + m2_height,
        "matrix vertical split height mismatch"
    );

    let mut out0 = Vec::new();
    let mut out1 = Vec::new();

    for i in 0..m1_height {
        out0.push(m[i].clone());
    }
    for i in m1_height..(m1_height + m2_height) {
        out1.push(m[i].clone());
    }
    (out0, out1)
}

fn compose_horizontal(m1: Vec<Vec<u128>>, m2: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    assert_eq!(
        m1.len(),
        m2.len(),
        "vertical size mismatch in horizontal composition"
    );
    let mut out: Vec<Vec<u128>> = Vec::new();
    for i in 0..m1.len() {
        let row = vec![m1[i].clone(), m2[i].clone()]
            .into_iter()
            .flatten()
            .collect::<Vec<u128>>();
        out.push(row);
    }
    out
}

fn compose_vertical(m1: Vec<Vec<u128>>, m2: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    let mut col_count = 0;
    let mut out: Vec<Vec<u128>> = Vec::new();
    for v in m1 {
        if col_count == 0 {
            col_count = v.len();
        }
        assert_eq!(col_count, v.len(), "column count mismatch");
        out.push(v);
    }
    for v in m2 {
        assert_eq!(col_count, v.len(), "column count mismatch");
        out.push(v);
    }
    out
}

// sum of absolute values of vector
fn l1_norm(v: Vec<u128>) -> u128 {
    let mut sum = 0_u128;
    for i in v {
        sum = (sum + i) % F;
    }
    sum
}

// square root of sum of squared vector values
fn l2_norm(v: Vec<u128>) -> u128 {
    let mut sum = 0_u128;
    println!("{}", v.len());
    for i in v {
        sum += (i * i) % F;
    }
    println!("{}", sum);
    let a = Fr::from(sum);
    println!("{}", a);
    if a.legendre().is_qnr() {
        panic!("non-quadratic residue");
    }
    println!("aaaa");
    let k = a.sqrt().unwrap().0 .0[0];
    println!("{}", k);
    u128::try_from(k).unwrap()
    // assert_eq!(sum % 2, 0, "expected l2 sum to be even");
    // if sum.pow(exp)
    // sum.sqrt()
    // a^((p-1)/2)
}

// max vector value
fn l_max_norm(v: Vec<u128>) -> u128 {
    let mut max = 0_u128;
    for i in v {
        if i > max {
            max = i;
        }
    }
    max
}

fn vec_matrix_mul(v: Vec<u128>, m: Vec<Vec<u128>>) -> Vec<u128> {
    let mut out = Vec::new();
    for r in m {
        out.push(l1_norm(vec_mul(&v, &r)));
    }
    out
}

fn matrix_mul(m1: Vec<Vec<u128>>, m2: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    assert_eq!(m1.len(), m2.len(), "matrix mul row count mismatch");
    let mut out = Vec::new();
    for i in 0..m1.len() {
        out.push(vec_mul(&m1[i], &m2[i]));
    }
    out
}

fn vec_mul(v1: &Vec<u128>, v2: &Vec<u128>) -> Vec<u128> {
    assert_eq!(v1.len(), v2.len(), "vector mul length mismatch");
    let mut out = Vec::new();
    for i in 0..v1.len() {
        out.push((v1[i] * v2[i]) % F);
    }
    out
}

// this addition happens collumn wise 🙄
fn vec_matrix_add(v: Vec<u128>, m: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    let mut out = Vec::new();
    for x in 0..m.len() {
        out.push(m[x].iter().map(|i| (i * v[x]) % F).collect::<Vec<u128>>());
    }
    out
}

fn matrix_add(m1: Vec<Vec<u128>>, m2: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    assert_eq!(m1.len(), m2.len(), "matrix add row count mismatch");
    let mut out = Vec::new();
    for i in 0..m1.len() {
        out.push(vec_add(&m1[i], &m2[i]));
    }
    out
}

fn vec_add(v1: &Vec<u128>, v2: &Vec<u128>) -> Vec<u128> {
    assert_eq!(v1.len(), v2.len(), "vector add length mismatch");
    let mut out = Vec::new();
    for i in 0..v1.len() {
        out.push((v1[i] + v2[i]) % F);
    }
    out
}

fn scalar_vec_mul(s: u128, v: &Vec<u128>) -> Vec<u128> {
    let mut out = Vec::new();
    for r in v {
        out.push((r * s) % F);
    }
    out
}

fn scalar_matrix_mul(s: u128, m: &Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    let mut out = Vec::new();
    for r in m {
        out.push(r.iter().map(|v| (v * s) % F).collect::<Vec<u128>>());
    }
    out
}

fn print_matrix(m: &Vec<Vec<u128>>) {
    for r in m {
        for v in r {
            print!("{:x}, ", v);
        }
        println!("");
    }
}
