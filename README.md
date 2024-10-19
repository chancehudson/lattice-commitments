# lattice-commitments [![Build](https://img.shields.io/circleci/build/github/chancehudson/lattice-commitments/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/lattice-commitments/tree/main) [![Docs](https://img.shields.io/docsrs/lattice-commitments)](https://docs.rs/lattice-commitments) [![Version](https://img.shields.io/crates/v/lattice-commitments)](https://crates.io/crates/lattice-commitments)

Structured lattice commitments based on [Baum et al.](https://eprint.iacr.org/2016/997.pdf)

## Usage

Clone the repo and run `cargo run`.

Change the configuration variables in [`main.rs`](./src/main.rs#L19) to adjust the base field and polynomial ring degree.

## Example output

```
Base field cardinality: 2013265921 (baby bear 32 bit)
Polynomial ring: ℤ[X]/<X^1024 + 1>

Committing to 1 polynomials, each containing 1024 coefficients:
1932515041,986406269,1994457158,1736428229,1385639216,381799741,1118301230,1918594090,312175584,1419926446,1548610658,1813929064,309965431,1101322493,400400652,1279971509,39382790,1084692826,513680150,3615292,614896213,774796250,752471911,733117172,1338984...

Opening commitment with secret vector (3 polynomials):
0,1,1,0,1,1,1,0,0,1,0,0,0,0,0,1,0,1,1,0,1,1,1,1,1,1,1,0,0,1,0,0,1,0,1,0,0,0,1,0,1,0,1,1,1,1,0,1,0,1,0,1,0,0,1,1,1,0,1,1,1,1,0,1,1,1,1,1,0,0,0,0,0,1,1,1,0,1,1,0,1,0,0,1,1,0,1,1,1,1,1,0,0,0,1,0,0,1,1,1,0,0,0,1,1,0,1,0,1,0,0,1,0,1,1,0,1,1,1,0,0,1,0,0,1,1,1,0,...

Commitment opening is valid!

Commitment size: 8388608 bytes
Public parameters size: 24576 bytes
Secret size: 12288 bytes
```
