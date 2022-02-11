# nixy

## Development

Please use a stable Rust compiler for normal operation.

```
nix-shell -p cargo

# for info from standard library follow
https://nnethercote.github.io/perf-book/profiling.html
# or maybe use the nix package?

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

# if we want panic abort. -Z build-std=panic_abort,std

# https://nnethercote.github.io/perf-book/profiling.html
# https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html
perf record --call-graph=dwarf 
perf record --call-graph=lbr
perf report --hierarchy -M intel

nix-shell -p valgrind kcachegrind graphviz

valgrind --tool=massif ./target/release/nixy

# https://valgrind.org/docs/manual/cg-manual.html
valgrind --tool=cachegrind --branch-sim=yes ./target/release/nixy
cg_annotate <filename>

# https://kcachegrind.github.io/html/Home.html
kcachegrind cachegrind.out.24077

# https://valgrind.org/docs/manual/cl-manual.html
cargo +nightly build -Z build-std --target x86_64-unknown-linux-gnu --release

cargo clean && RUSTFLAGS="-C target-cpu=native" cargo build --release

valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes --branch-sim=yes ./target/release/nixy
kcachegrind callgrind.out.53710

cargo clean && RUSTFLAGS="-C target-cpu=native" cargo build --release
valgrind --tool=massif ./target/release/nixy
nix-shell -p massif-visualizer
massif-visualizer massif.out.59811

https://github.com/flamegraph-rs/flamegraph
nix-shell -p linuxPackages_latest.perf
cargo install flamegraph
cargo flamegraph
```