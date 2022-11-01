# Plonky2 implementation of the [Semaphore protocol](http://semaphore.appliedzkp.org/)

Used as an example in the ZKHack Plonky2 presentation.

## Compilation
```bash
rustup override set nightly # Requires nightly Rust
cargo test --release
```

```bash
RUST_BACKTRACE=1 cargo run --release --example aggregate_n -- -vv 
```

## Bugs
- `set_proof_with_pis_target`
  - ```
    thread 'main' panicked at 'itertools: .zip_eq() reached end of one iterator before the other', /Users/maxwill/.cargo/registry/src/github.com-1ecc6299db9ec823/itertools-0.10.5/src/zip_eq_impl.rs:48:13
    stack backtrace:
    0: std::panicking::begin_panic
    1: plonky2::iop::witness::Witness::set_proof_with_pis_target
    2: plonky2_semaphore::recursion::<impl plonky2_semaphore::access_set::AccessSet>::aggregate_n_signals
    3: aggregate_n::main
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    ```


