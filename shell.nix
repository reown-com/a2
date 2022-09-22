{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = with pkgs; [
    openssl
    pkg-config
  ];
}
