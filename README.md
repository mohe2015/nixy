# nixy

## Development

Please use a stable Rust compiler for normal operation.

```
# https://doc.rust-lang.org/rustc/profile-guided-optimization.html
rm -rf /tmp/pgo-data
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release --target=x86_64-unknown-linux-gnu

# repeat
./target/x86_64-unknown-linux-gnu/release/nixy

rustup component add llvm-tools-preview

~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

cargo build --release --target=x86_64-unknown-linux-gnu
time ./target/x86_64-unknown-linux-gnu/release/nixy

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -Cllvm-args=-pgo-warn-missing-function" cargo build --release --target=x86_64-unknown-linux-gnu
time ./target/x86_64-unknown-linux-gnu/release/nixy

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