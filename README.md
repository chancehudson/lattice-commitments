# lattice-commitments [![Build](https://img.shields.io/circleci/build/github/chancehudson/lattice-commitments/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/lattice-commitments/tree/main) [![Docs](https://img.shields.io/docsrs/lattice-commitments)](https://docs.rs/lattice-commitments) [![Version](https://img.shields.io/crates/v/lattice-commitments)](https://crates.io/crates/lattice-commitments)

Structured lattice commitments based on [Baum et al.](https://eprint.iacr.org/2016/997.pdf)

## Usage

Clone the repo and run `cargo run`.

Change the configuration variables in [`main.rs`](./src/main.rs#L19) to adjust the base field and polynomial ring degree.

## Example output

```
Base field cardinality: 2013265921
Polynomial ring: ℤ[X]/<X^1024 + 1>

Committing to 1 polynomials, each containing 1024 coefficients:
1503116503,29185606,733657253,514068810,1816164352,1254901626,1179271528,1497499468,1279180158,109278558,470996513,405397967,1368800749,1120666995,1937707247,1848642082,1071825372,615824097,464091358,1472071551,226895880,150622874,1453159958,1214125048,170...

Opening commitment with secret vector (3 polynomials):
1,1,0,1,1,1,1,1,1,1,0,0,0,0,0,1,1,0,1,1,0,1,1,0,1,1,1,1,1,0,1,0,1,1,1,0,1,1,1,1,0,0,1,1,0,0,0,1,0,1,1,1,0,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,0,0,1,1,1,0,1,0,0,0,1,0,1,1,1,1,0,1,0,1,1,1,1,0,0,1,0,1,1,0,1,1,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,0,1,0,1,0,1,1,1,0,1,0,1,1,...

Commitment opening is valid!

Commitment size: 8388608 bytes
Public parameters size: 24576 bytes
Secret size: 12288 bytes
```
