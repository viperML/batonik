let
  pkgs = import <nixpkgs> {};
in
  with pkgs;
    mkShell {
      packages = [
        go
        gopls
        go-tools
      ];
    }
