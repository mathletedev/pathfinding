{ pkgs ? import <nixpkgs> { }, lib ? pkgs.lib }:

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    libGL
    libxkbcommon
    xorg.libX11
    xorg.libXi
  ];

  LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
    "${pkgs.libGL}/lib"
    "${pkgs.libxkbcommon}/lib"
    "${pkgs.xorg.libX11}/lib"
    "${pkgs.xorg.libXi}/lib"
  ];
}
