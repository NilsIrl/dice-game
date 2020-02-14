with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "dice-game";
  buildInputs = [
    pkgs.ncurses
    pkgs.postgresql
  ];
}
