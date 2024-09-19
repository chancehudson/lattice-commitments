# lattice-commitments [![Build](https://img.shields.io/circleci/build/github/chancehudson/lattice-commitments/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/lattice-commitments/tree/main)

Structured lattice commitments based on [Baum et al.](https://eprint.iacr.org/2016/997.pdf)

## Usage

Clone the repo and run `cargo run`.

Change the configuration variables in [`main.rs`](./src/main.rs#L19) to adjust the base field and polynomial ring degree.

## Example output

```
Base field cardinality: 101
Polynomial ring: ℤ[X]/<X^16 + 1>

Committing to 2 polynomials, each containing 16 coefficients:
80,53,67,70,1,53,39,98,45,35,94,18,16,40,30,50,
29,29,14,11,61,30,93,42,81,91,35,98,90,25,75,50

Opening commitment with secret vector (8 polynomials):
96,0,18,22,34,82,35,98,23,28,17,32,39,90,52,
30,67,72,98,34,88,34,9,42,16,2,19,73,79,37,
62,99,31,18,19,41,42,86,16,32,30,56,70,3,33,
38,25,47,16,89,57,12,93,3,0,9,97,64,62,7,
54,46,13,24,3,63,19,53,57,98,37,57,16,53,98,
27,90,77,48,69,13,5,83,38,6,64,81,17,77,6,
11,45,3,86,90,14,39,68,91,95,83,46,25,10,86,
75,43,37,49,15,52,84,36,40,75,41,57,47,88,42

Commitment opening is valid!

Commitment size: 7168 bytes
Public parameters size: 3584 bytes
Secret size: 896 bytes
```
