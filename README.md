# Tiny Rust Hello

> A Super Small Hello World in Rust

This project is a proof of concept to myself that Rust can make very small executables with very readable code. Ideally, there will be no major differences in the `main.rs` file or command line to build regardless of target architecture.

- [x] minilib amd64 (768 bytes)
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
cargo build --release && ls -lah target/x86_64-unknown-linux-gnu/release/tiny-rust-hello && ./target/x86_64-unknown-linux-gnu/release/tiny-rust-hello 'mojo jojo'
```

```sh
# (aarch64) Build, show size, and run
cargo build --target aarch64-unknown-linux-gnu --release && ls -lah target/aarch64-unknown-linux-gnu/release/tiny-rust-hello && qemu-aarch64-static target/aarch64-unknown-linux-gnu/release/tiny-rust-hello 'mojo jojo'
```

## Related projects

- [min-sized-rust](https://github.com/johnthagen/min-sized-rust)
- [Making your own executable packer](https://fasterthanli.me/series/making-our-own-executable-packer/)
  - Excellent write up by Amos Wenger (fasterthanlime), [part 12](https://fasterthanli.me/series/making-our-own-executable-packer/part-12) was especially helpful to me
