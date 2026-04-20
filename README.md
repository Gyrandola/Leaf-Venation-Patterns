# Venation Pattern Generator


![Build Status](https://github.com/Gyrandola/Leaf-Venation-Patterns/actions/workflows/rust.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-v1.70%2B-orange?logo=rust&style=flat-square)
![Macroquad](https://img.shields.io/badge/made%20with-macroquad-blueviolet?style=flat-square)
![License](https://img.shields.io/github/license/Gyrandola/Leaf-Venation-Patterns?style=flat-square&color=blue)
![Repo Size](https://img.shields.io/github/repo-size/Gyrandola/Leaf-Venation-Patterns?style=flat-square)



A Rust implementation of the Space Colonization Algorithm for procedural leaf vein generation, based on [Runions et al. (2005)](https://algorithmicbotany.org/papers/venation.sig2005.pdf).

![Simulation Preview](media/preview.gif)

## Algorithm
The simulation iterates through a growth loop using two components: Auxins and Veins. Each auxin influences the single closest vein node within max distance..
Vein nodes calculate a normalized growth vector toward their influencing auxins and sprout new segments. When a vein reaches an auxin, the auxin is removed.
## Usage

Build from source
(requires [rust & cargo](https://rustup.rs/)).
```bash
git clone https://github.com/Gyrandola/Leaf-Venation-Patterns.git
cd Leaf-Venation-Patterns && cargo run --release
```

Alternatively download and run the [latest release](https://github.com/Gyrandola/Leaf-Venation-Patterns/releases).

## Dependencies
[Macroquad](https://macroquad.rs/)

## References
Runions, A., Fuhrer, M., Lane, B., Federl, P., Rolland-Lagan, A. G., & Prusinkiewicz, P. (2005). [Modeling and Visualization of Leaf Venation Patterns](https://algorithmicbotany.org/papers/venation.sig2005.pdf).
