{
  description = "A nix flake for working with vanilla rust";

  inputs = {
    self.submodules = true;
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let

        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

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
            vulkan-loader

          ]
          ++ linux_libs;

        nativeBuildInputs = with pkgs; [
          glfw
          cmake
          clang
          cargo
          rustc
        ];

        linkFlag = buildInputs;

        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath linkFlag}";
      in
      {

        # declaring the build with the naerskLib flake
        packages.default = craneLib.buildPackage {
          inherit nativeBuildInputs buildInputs LD_LIBRARY_PATH;
          src = ./.;

          gitSubModules = true;

          env = {
            RUSTFLAGS = "-C link-args=-Wl,-rpath,${pkgs.lib.makeLibraryPath linkFlag}";
          };

        };

        templates.default.path = ./.;

        devShell = pkgs.mkShell {
          inherit LD_LIBRARY_PATH;

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
