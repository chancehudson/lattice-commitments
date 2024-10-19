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
1359547193,340921440,683036976,237349484,130545256,1892940395,1002789220,1588260374,356191913,539134055,992954628,1181200044,397478076,1772157665,979316199,1456235978,1418093855,248218780,1951850829,597759998,32218409,978492533,1321779583,106310933,8609823...

Opening commitment with secret vector (3 polynomials):
0,1,1,1,1,0,1,1,1,0,0,0,1,0,1,1,1,1,1,1,1,1,0,0,0,0,0,1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,1,1,1,1,0,1,1,1,0,0,0,1,1,1,0,0,1,0,0,1,0,1,0,1,1,1,0,0,1,0,1,1,1,1,0,0,0,1,0,0,0,0,1,0,0,0,0,1,1,1,0,1,1,1,0,1,0,0,1,1,1,1,1,1,1,1,0,0,0,1,0,0,1,0,1,1,1,0,0,1,1,0,0,0,1,1,...

Commitment opening is valid!

Commitment size: 8192 bytes
Public parameters size: 24576 bytes
Secret size: 12288 bytes
```
