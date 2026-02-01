{
  description = "The Pregen game engine";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, rust-overlay, ... }: let
    overlays = [(import rust-overlay)];
    systems = ["x86_64-linux" "aarch64-darwin"];
    forAllSystems = nixpkgs.lib.genAttrs systems;

    pkgsFor = system: import nixpkgs {
      inherit system overlays;
    };
  in {
    devShells = forAllSystems (system: let
      pkgs = pkgsFor system;
      rustToolchain = pkgs.rust-bin.stable."1.92.0".default.override {
        extensions = ["rust-src" "rust-std" "rust-analyzer"];
        targets = ["x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu"];
      };
    in {
      # TODO: fix darwin build
      default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          wgpu-utils tracy lldb valgrind

          # Build dependencies
          rustToolchain clang mold
          stdenv.cc.cc.lib pkg-config
          unzip extra-cmake-modules cmake

          # Vulkan
          vulkan-headers vulkan-loader
          vulkan-tools vulkan-extension-layer
          vulkan-tools-lunarg vulkan-validation-layers

          # Wayland
          wayland wayland-protocols wayland-scanner libxkbcommon

          # X11
          xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi xorg.libXinerama xorg.libxcb
        ];

        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          export RUSTFLAGS="$RUSTFLAGS -C linker=${pkgs.clang}/bin/clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
        '';
      };

      # FIXME: wine is not available on darwin???
      windows = pkgs.pkgsCross.mingwW64.mkShell rec {
        nativeBuildInputs = with pkgs; [
          wineWowPackages.stable
          unzip extra-cmake-modules
          pkgsCross.mingwW64.stdenv.cc
          rustToolchain cmake pkg-config
        ];

        buildInputs = with pkgs; [
          pkgsCross.mingwW64.vulkan-loader
          pkgsCross.mingwW64.vulkan-headers
          pkgsCross.mingwW64.windows.pthreads
        ];

        shellHook = ''
          export WINEPREFIX="$PWD/.direnv/wine-prefix"
          export CARGO_BUILD_TARGET="x86_64-pc-windows-gnu"

          export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER="${pkgs.wineWowPackages.stable}/bin/wine64";
          export AR_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-ar"
          export CC_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
          export CXX_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-g++"
          export RC_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-windres"
          export RUSTFLAGS="${pkgs.lib.concatStringsSep " " (pkgs.lib.map (p: "-L native=${p}/lib") buildInputs)}"
          export LD_LIBRARY_PATH="${builtins.toString (pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs))}"
          export DYLD_LIBRARY_PATH="${builtins.toString (pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs))}"

          export CMAKE_AR="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}ar"
          export CMAKE_C_COMPILER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc"
          export CMAKE_CXX_COMPILER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}c++"
          export CMAKE_RC_COMPILER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}windres"
        '';
      };
    });
  };
}
