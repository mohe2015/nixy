with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc.all cargo rust-analyzer rustfmt
  ];
  buildInputs = [ #
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
