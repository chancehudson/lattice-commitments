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
54,47,74,97,43,9,77,91,50,36,77,71,17,33,15,62,
7,54,58,9,96,4,62,59,99,7,68,99,83,17,83,7

Opening commitment with secret vector (8 polynomials):
89,83,37,90,55,63,16,22,11,62,65,61,73,15,19,
65,85,66,5,18,52,64,49,54,61,74,61,45,4,11,
83,7,84,76,25,85,72,4,18,16,88,97,52,35,47,
52,94,94,99,82,5,26,0,39,53,45,23,75,45,50,
43,25,54,20,32,40,55,7,26,73,60,22,73,23,48,
69,78,28,70,41,44,87,83,85,31,76,77,1,10,8,
59,9,4,29,66,75,5,8,26,65,24,38,8,73,98,
26,76,97,22,88,81,70,34,95,47,38,38,17,20,77

Commitment opening is valid!

Commitment size: 512 bytes
Public parameters size: 256 bytes
Secret size: 64 bytes
```
