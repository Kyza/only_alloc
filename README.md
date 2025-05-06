# only_alloc

`only_alloc` wraps another allocator and disables deallocating memory within it.

It can be used either as a global allocator or as an allocator from the `alloc_trait` STD feature.

```rs
use mimalloc::MiMalloc;
use only_alloc::GlobalOnlyAlloc;

#[global_allocator]
static ONLY_MIMALLOC: GlobalOnlyAlloc<MiMalloc> = GlobalOnlyAlloc(MiMalloc);
```

## Installation

```bash
# cargo add only_alloc
cargo add https://github.com/Kyza/only_alloc
```

## Why?

Maybe you have a use for this because I don't. It might be microscopically faster for CLI tools. I simply enjoyed writing it.
