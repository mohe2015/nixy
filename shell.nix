with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc.all
    cargo
    rust-analyzer
    rustfmt
    google-java-format
    openjdk17
  ];
  buildInputs = [ ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
