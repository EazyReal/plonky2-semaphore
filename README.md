# Plonky2 implementation of the [Semaphore protocol](http://semaphore.appliedzkp.org/)

Used as an example in the ZKHack Plonky2 presentation.

- recursive should not use merkle path optimization (for its a circuit)

## TODO
- see how fri (recursive verifier's fri) is implemented
  - i.e. find why `at.0` and `a.0` have different lengths.
- open an issue on plonky2_semaphore 
- report
  - recursive SNARKs condition (preprocessing and aggregatable), 10 min
  - preprocessing recursive SNARK by picture, 5 min
  - preprocessing recursive SNARK bycode (benchrecursion or this) 20 min

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

## Test
- fu: what if standard config is not suitable for recursion in small-sized circuit?
  - recursive proof 1 proof (check if same)

## Known 
- zipeq is ok
  - the benchmark works
- vd s are the same (by Derived Eq)
  - tested see commit 397d7fe
- vd0 can be used as vd1's verifying key
  - tested see commit 9d21005
- test if use vds instead of vd to aggregate_n can work
  - the vds are all the same (397d7fe)
- recurse one single proof yields the same error
```
before recursion
before
12, 12
pi targets: [VirtualTarget { index: 18299 }, VirtualTarget { index: 18300 }, VirtualTarget { index: 18301 }, VirtualTarget { index: 18302 }, VirtualTarget { index: 18303 }, VirtualTarget { index: 18304 }, VirtualTarget { index: 18305 }, VirtualTarget { index: 18306 }, VirtualTarget { index: 18307 }, VirtualTarget { index: 18308 }, VirtualTarget { index: 18309 }, VirtualTarget { index: 18310 }]
public inputs: [8994285297209132218, 9205088315991535708, 5684298355530720120, 12031286025610392011, 6359566002007280689, 830954550583560438, 16311305749455435813, 753546054450889101, 14955740701851009686, 17217178120894370469, 16250875237364447440, 14579114485892679215]
after
i = 0
j = 0
j = 1
thread 'main' panicked at 'itertools: .zip_eq() reached end of one iterator before the other', /Users/maxwill/.cargo/registry/src/github.com-1ecc6299db9ec823/itertools-0.10.5/src/zip_eq_impl.rs:48:13
stack backtrace:
   0: std::panicking::begin_panic
   1: plonky2::fri::witness_util::set_fri_proof_target
   2: plonky2::iop::witness::Witness::set_proof_with_pis_target
   3: plonky2_semaphore::recursion::<impl plonky2_semaphore::access_set::AccessSet>::recursive_proof
   4: recursion1::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

## Found Length Mismatch
```
before recursion
recursive_proof use config from verifier data.
before
12, 12
pi targets: [VirtualTarget { index: 18299 }, VirtualTarget { index: 18300 }, VirtualTarget { index: 18301 }, VirtualTarget { index: 18302 }, VirtualTarget { index: 18303 }, VirtualTarget { index: 18304 }, VirtualTarget { index: 18305 }, VirtualTarget { index: 18306 }, VirtualTarget { index: 18307 }, VirtualTarget { index: 18308 }, VirtualTarget { index: 18309 }, VirtualTarget { index: 18310 }]
public inputs: [12044919867727768343, 5567262085942038047, 6379617696266071128, 12527335638119924889, 8407020272154425921, 15900843360727847529, 2008799932033209496, 807360721208600325, 17230018333962668289, 12903009343789509794, 17192247947549007729, 245364673252995857]
after
i = 0
j = 0
a.0.siblings len 86, 86
a.1.siblings len 13, 13
j = 1
a.0.siblings len 135, 139
a.1.siblings len 13, 13
thread 'main' panicked at 'itertools: .zip_eq() reached end of one iterator before the other', /Users/maxwill/.cargo/registry/src/github.com-1ecc6299db9ec823/itertools-0.10.5/src/zip_eq_impl.rs:48:13
stack backtrace:
   0: std::panicking::begin_panic
   1: plonky2::fri::witness_util::set_fri_proof_target
   2: plonky2::iop::witness::Witness::set_proof_with_pis_target
   3: plonky2_semaphore::recursion::<impl plonky2_semaphore::access_set::AccessSet>::recursive_proof
   4: recursion1::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```