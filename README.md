# only_alloc

[![github.com](https://img.shields.io/badge/github-kyza/only_alloc-fffffe?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/kyza/only_alloc)
[![crates.io](https://img.shields.io/crates/v/only_alloc.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/only_alloc)
[![docs.rs](https://img.shields.io/badge/docs.rs-only_alloc-353535?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/only_alloc)

`only_alloc` wraps another allocator and disables deallocating memory within it.

It can be used either as a global allocator or as an allocator from `#![feature(allocator_api)]`.

It can be used with `no_std` environments and it contains zero dependencies.

```rs
use mimalloc::MiMalloc;
use only_alloc::GlobalOnlyAlloc;

#[global_allocator]
static ONLY_MIMALLOC: GlobalOnlyAlloc<MiMalloc> = GlobalOnlyAlloc(MiMalloc);
```

## Installation

```bash
cargo add only_alloc
```

## Why?

Maybe you have a use for this because I don't. It might be microscopically faster for CLI tools. I simply enjoyed writing it.
