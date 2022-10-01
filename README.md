# Computer Graphics Geometric Algebra Library

## Introduction
**cggeomalg** is a low-dimensional geometric algebra aimed at 
real-time computer graphics, real-time physics, and simulations 
in the language of real-valued Clifford algebras.

## Getting Started
To use the library in your project, add **cggeomalg** as a dependency in your 
`Cargo.toml` file:
```toml
[dependencies.cggeomalg]
version = "0.2.1"
```
After that, place the crate declaration in either your `lib.rs` file or 
your `main.rs` file
```rust
extern crate cggeomalg;
```
The library aims to be as platform agnostic as possible. By default, the library 
supports any environment that supports the standard library `std`, but because 
the library does not require any allocations, it also supports environments built
on either `core` or `alloc`. `std` is the default support feature, but you can add
support for either `alloc` or `core` by adding
```toml
[dependencies.cglinalg]
# Use `cggeomalg` with the `alloc` crate
features = ["alloc"]
```
for the `alloc` crate, or
```toml
features = ["core"]
```
for the `core` crate.

