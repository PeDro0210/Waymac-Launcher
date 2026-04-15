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

        linux_libs =
          if pkgs.stdenv.isLinux then
            with pkgs;
            [

              libX11
              libXcursor
              libXi
              libXrandr
              libxkbcommon
              libX11

              alsa-lib
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
          ++ linux_libs;

        nativeBuildInputs = with pkgs; [
          glfw
          cmake
          clang
          cargo
          rustc
        ];

        linkFlag = nativeBuildInputs ++ buildInputs;

        LD_LIBRARY_PATH =
          if pkgs.stdenv.isLinux then
            builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs
          else
            "";

      in
      {

        # declaring the build with the naerskLib flake
        packages.default = naerskLib.buildPackage {
          inherit nativeBuildInputs buildInputs LD_LIBRARY_PATH;
          src = ./.;

          env.RUSTFLAGS = "-C link-args=-Wl,-rpath,${pkgs.lib.makeLibraryPath linkFlag}";

        };

        templates.default.path = ./.;

        devShell = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs LD_LIBRARY_PATH;

          packages = with pkgs; [
            cargo
            bacon
            rust-analyzer
            clippy
            rustfmt
            taplo # lsp for cargo.toml
          ];

        };

      }
    );
}
