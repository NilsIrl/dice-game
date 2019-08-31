
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "dice-game";
  buildInputs = [
    pkgs.cargo
	pkgs.ncurses
	pkgs.openssl
	pkgs.sqlite
  ];

  shellHook = ''
    export OPENSSL_DIR="${openssl.dev}"
	export OPENSSL_LIB_DIR="${openssl.out}/lib"
  '';
}
