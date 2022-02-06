# nix-shell --pure
with import <nixpkgs> {};
llvmPackages_13.libcxxStdenv.mkDerivation {
  name = "clang-nix-shell";
  nativeBuildInputs = [ flex bison ];
  buildInputs = [ /* add libraries here */ ];
}
