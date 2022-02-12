with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc.all
    cargo
    rust-analyzer
    rustfmt
    google-java-format
  ];
  buildInputs = [ ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
