{
  description = "Cross compiling a rust program using rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      localSystem:
      let
        # Replace with the system you want to build for
        crossSystem = "x86_64-linux";

        pkgs = import nixpkgs {
          inherit crossSystem localSystem;
          overlays = [ (import rust-overlay) ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

        crateExpression =
          {
            openssl,
            libiconv,
            lib,
            pkg-config,
            stdenv,
          }:
          craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            nativeBuildInputs = [
              pkgs.expat
              pkgs.fontconfig
              pkgs.freetype
              pkgs.freetype.dev
              pkgs.libGL
              pkg-config
              pkgs.vulkan-loader
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
              pkgs.libxkbcommon

              pkgs.alsa-lib
              pkgs.wayland # To use the wayland feature

            ];

            buildInputs = [
              # Add additional build inputs here
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
              pkgs.libxkbcommon

              pkgs.alsa-lib
              pkgs.wayland # To use the wayland feature

              openssl
            ];

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
              # Add additional build inputs here
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
              pkgs.libxkbcommon

              pkgs.alsa-lib
              pkgs.wayland # To use the wayland feature

              openssl
            ]}";
          };

        waymac_launcher = pkgs.callPackage crateExpression { };
      in
      {
        checks = {
          inherit waymac_launcher;
        };

        packages.default = waymac_launcher;
      }
    );
}
