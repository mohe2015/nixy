# nixy

## Development

```
# https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
RUSTFLAGS=-Zsanitizer=memory cargo +nightly run -Z build-std --target x86_64-unknown-linux-gnu --release

# https://nnethercote.github.io/perf-book/profiling.html
# https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html
perf record --call-graph=dwarf 
perf record --call-graph=lbr
perf report --hierarchy -M intel

nix-shell -p valgrind
 
valgrind --tool=massif ./target/release/nixy

# https://valgrind.org/docs/manual/cg-manual.html
valgrind --tool=cachegrind --branch-sim=yes ./target/release/nixy
cg_annotate <filename>

# lots of mispredicted indirect branches
nix-shell -p kcachegrind
kcachegrind cachegrind.out.24077

# https://valgrind.org/docs/manual/cl-manual.html
valgrind --tool=callgrind ./target/release/nixy
callgrind_annotate [options] callgrind.out.<pid>
# https://kcachegrind.github.io/html/Home.html

https://github.com/flamegraph-rs/flamegraph
nix-shell -p linuxPackages_latest.perf
cargo install flamegraph
cargo flamegraph
```