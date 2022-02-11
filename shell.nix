with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc.all cargo rust-analyzer
  ];
  buildInputs = [ #
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
