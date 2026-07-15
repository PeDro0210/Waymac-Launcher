{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      nixpkgs,
      crane,
      ...
    }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs
          [
            "x86_64-linux"
            "aarch64-darwin"
          ]
          (
            system:
            let
              pkgs = nixpkgs.legacyPackages.${system};

              linux_libs =
                if pkgs.stdenv.isLinux then
                  with pkgs;
                  [

                    xorg.libX11
                    xorg.libXcursor
                    xorg.libXi
                    xorg.libXrandr
                    libxkbcommon

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
                makeWrapper
              ];

            in
            function {
              inherit
                pkgs
                buildInputs
                nativeBuildInputs
                ;

            }
          );
    in
    {

      # declaring the build with the naerskLib flake
      packages = forAllSystems (
        {
          pkgs,
          nativeBuildInputs,
          buildInputs,
        }:
        {
          default =
            let
              craneLib = crane.mkLib pkgs;
              lib = pkgs.lib;
            in
            craneLib.buildPackage {
              inherit nativeBuildInputs buildInputs;
              src = ./.;

              env = {
                RUSTFLAGS = "-C link-args=-Wl,-rpath,${pkgs.lib.makeLibraryPath buildInputs}";
              };

              InstallPhase = ''
                makeWrapper $out/bin/waymac_launcher $wrapperfile
                --set LD_LIBRARY_PATH ${lib.makeBinPath buildInputs}
              '';

            };
        }
      );

      templates.default.path = ./.;

      devShells = forAllSystems (
        {
          pkgs,
          buildInputs,
          nativeBuildInputs,
        }:
        {
          default =
            let
              LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
            in
            pkgs.mkShell {
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

    };
}
