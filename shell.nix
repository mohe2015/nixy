# nix-shell --pure
with import <nixpkgs> {};
llvmPackages_13.libcxxStdenv.mkDerivation {
  name = "clang-nix-shell";
  nativeBuildInputs = [ antlr4 ];
  buildInputs = [ /* add libraries here */ ];
}
