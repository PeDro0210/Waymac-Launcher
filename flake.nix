{
  description = "A nix flake for working with vanilla rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      naersk,
      flake-utils,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let

        pkgs = nixpkgs.legacyPackages.${system};
        naerskLib = pkgs.callPackages naersk { };

        base_lib = with pkgs; [
        ];

        std_bin = with pkgs; [
          glfw
          cmake
          clang
          pkg-config
          cargo
          bacon
          rustc
          rust-analyzer
          clippy
          rustfmt
          taplo # lsp for cargo.toml
          bacon
        ];

        link_flag = base_lib ++ std_bin;
      in
      {

        # declaring the build with the naerskLib flake
        packages.default = naerskLib.buildPackage {
          src = ./.;
          buildInputs = base_lib;
          nativeBuildInputs = std_bin;

          env.RUSTFLAGS = "-C link-args=-Wl,-rpath,${pkgs.lib.makeLibraryPath link_flag}";
        };

        templates.default.path = ./.;

        devShell = pkgs.mkShell {
          packages = std_bin;
        };

      }
    );
}
