use rand::prelude::*;

fn main() {
    let mut x: Vec<u128> = Vec::new();
    for _ in 0..16 {
        x.push(rand_field());
    }
    commit(x);

    println!("Hello, world!");
}

static F: u128 = 2_u128.pow(64) + 1;

fn rand_field() -> u128 {
    rand::random::<u128>() % F
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
        let mut row: Vec<u128> = Vec::new();
        for _ in 0..c {
            row.push(rand_field());
        }
        out.push(row);
    }
    out
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
    for i in v {
        sum += (i * i) % F;
    }
    assert_eq!(sum % 2, 0, "expected l2 sum to be even");
    // sum.sqrt()
    // a^((p-1)/2)
    0
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

fn vec_matrix_mul(v: Vec<u128>, m: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    let mut out = Vec::new();
    for r in m {
        out.push(vec_mul(&v, &r));
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

fn vec_matrix_add(v: Vec<u128>, m: Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    let mut out = Vec::new();
    for r in m {
        out.push(vec_add(&v, &r));
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

fn print_matrix(m: &Vec<Vec<u128>>) {
    for r in m {
        for v in r {
            print!("{:x}, ", v);
        }
        println!("");
    }
}

fn commit(x: Vec<u128>) -> Vec<Vec<u128>> {
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
    // modinv of 64 in F = 2^64 - 2^58
    // beta must be less than modinv(64)

    let beta = 100_u128;
    let r = vec![0; k]
        .into_iter()
        .map(|_| rand_field() % beta)
        .collect::<Vec<u128>>();

    let inter1 = vec_matrix_mul(r, compose_vertical(alpha_1, alpha_2));

    // here we're using a zero vector of length n-l
    // this is different than what the paper specifies
    // the paper specifies addition between a matrix and vector
    // of mismatched sizes
    // awaiting response from authors
    let inter2 = vec![vec![0; k - l], x].concat();
    let commitment = vec_matrix_add(inter2, inter1);

    print_matrix(&commitment);

    commitment
}
