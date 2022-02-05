# nix-shell --pure
with import <nixpkgs> {};
llvmPackages_13.libcxxStdenv.mkDerivation {
  name = "clang-nix-shell";
  buildInputs = [ /* add libraries here */ ];
}
