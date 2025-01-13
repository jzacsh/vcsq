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


    #
    # Packages specific to vcsq
    #

    #
    # Packages specific to vcsq's domain
    #
    git,
    mercurial,
    jujutsu,

    #
    # Packages specific to vcsq's ci/cd tasks
    #
    cargo-llvm-cov, # for test coverage
    cargo-binutils,
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      rustup
      rustPlatform.bindgenHook
    ];
    # libraries here
    buildInputs = [
    ];
    RUSTC_VERSION = overrides.toolchain.channel;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
      export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
    '';
  }
) { }
