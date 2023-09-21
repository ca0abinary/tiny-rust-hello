# Tiny Rust Hello

> A Super Small Hello World in Rust

This project is a proof of concept to myself that Rust can make very small executables with very readable code. Ideally, there will be no major differences in the `main.rs` file or command line to build regardless of target architecture.

- [x] minilib amd64 (512 bytes)
- [ ] minilib arm64
- [ ] minilib aarch64
- [ ] minilib riscv

## Objective

To make this code run on multiple architectures at less than 1_024 bytes compiled:

```rust
print("Hello, world!\n");
exit(0)
```

## Direction

- Build a module `minilib` with one module per architecture
- Choose architectures in `minilib/mod.rs` using the [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html) feature

## Build command

```sh
# Build only
cargo build --release
```

```sh
# Build, show size, and run
cargo build --release && ls -lah target/release/smallrs && ./target/release/smallrs
```
