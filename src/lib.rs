/*!
# Computer Graphics Geometric Algebra Library

## Introduction
**cggeomalg** is a low-dimensional geometric algebra aimed at 
real-time computer graphics, real-time physics, and simulations 
in the language of real-valued Clifford algebras.

## Getting Started
To use the library in your project, add **cggeomalg** as a dependency in your 
`Cargo.toml` file:
```ignore
[dependencies]
cggeomalg = "0.2.0"
```
After that, place the crate declaration in either your `lib.rs` file or 
your `main.rs` file
```rust
extern crate cggeomalg;
```

*/

#![no_std]
extern crate core;

extern crate approx;
extern crate num_traits;


mod coordinates;

pub mod scalar;
pub mod e2ga;
pub mod e3ga;

