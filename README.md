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
1124335506,149818619,1601233164,205230700,1475683794,659969253,282573766,1962836835,893421200,512127742,185294682,780911461,82017087,1974806903,1904974401,1567299238,333737732,433543762,1300045461,1587965164,52563316,254413285,194076911,358654244,178023259...

Opening commitment with secret vector (3 polynomials):
0,0,0,1,1,1,0,1,1,0,1,0,0,1,0,0,0,1,1,0,1,0,1,0,0,1,0,1,0,0,0,1,1,1,0,1,0,1,1,1,1,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,1,0,0,0,1,1,1,1,0,1,0,0,1,1,1,1,0,0,0,0,0,0,1,0,0,0,1,1,1,1,1,0,1,1,0,1,0,0,0,1,0,1,0,1,1,0,1,1,0,1,0,1,0,0,1,0,1,1,1,0,1,1,1,0,0,1,0,1,1,1,0,1,...

Commitment opening is valid!

Commitment size: 8192 bytes
Public parameters size: 24576 bytes
Secret size: 3 bytes
```
