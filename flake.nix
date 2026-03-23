{
  description = "A nix flake for working with vanilla rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      nixpkgs,
      naersk,
      ...
    }:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
            };
          in
          f {

            inherit system pkgs;
            packages = with pkgs; [
              cargo
              clippy
              rustfmt
              bacon
              taplo # lsp for cargo.toml
            ];
          }
        );

    in
    {
      packages = forEachSupportedSystem (
        {
          pkgs,
          ...
        }:
        let
          naerskLib = pkgs.callPackages naersk { };
          targetSpecificBuildInputs =
            #TODO: check if xorg can be checked and wayland
            if pkgs.stdenv.isLinux then
              with pkgs;
              [
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                libxkbcommon

                alsa-lib
                xorg.libX11
                wayland # To use the wayland feature
              ]
            else
              [

              ];

          buildInputs =
            with pkgs;
            [
              expat
              fontconfig
              freetype
              freetype.dev
              libGL
              pkg-config
            ]
            ++ targetSpecificBuildInputs;

          nativeBuildInputs = with pkgs; [
            glfw
            cmake
            clang
            pkg-config

            rustc
            rust-analyzer
          ];

        in
        {
          default = naerskLib.buildPackage {
            inherit nativeBuildInputs buildInputs;

            src = ./.;
            env.RUSTFLAGS = "-C link-args=-Wl,-rpath,${pkgs.lib.makeLibraryPath buildInputs}";

          };
        }
      );

      templates.default.path = ./.;

      devShells = forEachSupportedSystem (
        { pkgs, packages, ... }:
        {
          default = pkgs.mkShell {
            inherit packages;
          };
        }
      );

    };
}
