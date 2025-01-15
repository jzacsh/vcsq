# adapted from https://wiki.nixos.org/wiki/Rust#Installation_via_rustup
{
    pkgs ? import <nixpkgs> { },
}:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in
pkgs.callPackage (
  {
    stdenv,
    mkShell,
    rustup,
    rustPlatform,
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      rustup
      rustPlatform.bindgenHook

      #
      # Packages specific to vcsq
      #
      pkgs.git
      pkgs.mercurial
      pkgs.jujutsu
      pkgs.cargo-llvm-cov # for test coverage
      pkgs.grcov # for test coverage
    ];
    # libraries here
    buildInputs = [
    ];
    RUSTC_VERSION = overrides.toolchain.channel;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
      export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"

      export RUSTFLAGS="''${RUSTFLAGS:+$RUSTFLAGS }-Cinstrument-coverage -Ddeprecated -Dwarnings "
    '';
  }
) { }
