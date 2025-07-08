with import <nixpkgs> { };
mkShell {
  packages = [
    rustc
    cargo
    rustfmt
  ];

  env = {
    RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  };
}
