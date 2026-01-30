{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    gcc
    pkg-config
    rust-analyzer
  ];

  buildInputs = with pkgs; [
    openssl
    zlib
    sqlite
  ];

  shellHook = ''
    export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    echo "🦀 Blazingly fast shell activated!"
    rustc --version
  '';
}
