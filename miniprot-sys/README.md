![https://crates.io/crates/miniprot-sys](https://img.shields.io/crates/v/minimap2.svg)
![https://docs.rs/miniprot-sys/latest/miniprot-sys/](https://img.shields.io/docsrs/minimap2)

# Introduction
A rust FFI library for [miniprot](https://github.com/lh3/miniprot/). Low-level bindings. For higher-level usage, please see miniprot-rs (coming soon).

# How to use
## Requirements
```toml
miniprot-sys = "0.1.0"
```

Tested with rustc 1.64.0 and nightly.

# Citation
You should cite the minimap2 papers if you use this in your work.

> Heng Li, Protein-to-genome alignment with miniprot, *Bioinformatics*, 2023;, btad014, https://doi.org/10.1093/bioinformatics/btad014

# Changelog
## 0.1.0
* Initial Rust FFI Bindings

# TODO
* Possible to decouple from pthreads?

# Funding
![Genomics Aotearoa](info/genomics-aotearoa.png)
