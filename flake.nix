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
        extensions = ["rust-src" "rust-std" "rust-analyzer" "clippy"];
        targets = ["x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu"
                   "wasm32-unknown-unknown" "wasm32-unknown-emscripten"];
      };
    in {
      default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          wgpu-utils tracy lldb

          # Build dependencies
          rustToolchain clang mold
          stdenv.cc.cc.lib pkg-config
          unzip extra-cmake-modules cmake

          # Vulkan
          vulkan-headers vulkan-loader
          vulkan-validation-layers vulkan-tools
        ] ++ lib.optionals (pkgs.stdenv.isLinux) [
          vulkan-tools-lunarg vulkan-extension-layer # shite
          libX11 libXcursor libXrandr libXi libXinerama libxcb
          wayland wayland-protocols wayland-scanner libxkbcommon
        ];

        shellHook = ''
          export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"
        '' + pkgs.lib.optionalString (pkgs.stdenv.isLinux) ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          export RUSTFLAGS="$RUSTFLAGS -C linker=${pkgs.clang}/bin/clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
        '' + pkgs.lib.optionalString (pkgs.stdenv.isDarwin) ''
          export DYLD_LIBRARY_PATH="$DYLD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
        '';
      };

      # FIXME: wine is not available on darwin???
      windows = pkgs.pkgsCross.mingwW64.mkShell rec {
        nativeBuildInputs = with pkgs; [
          wineWow64Packages.stable
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

          export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER="${pkgs.wineWow64Packages.stable}/bin/wine";
          export AR_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-ar"
          export CC_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
          export CXX_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-g++"
          export RC_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-windres"
          export RUSTFLAGS="${pkgs.lib.concatStringsSep " " (pkgs.lib.map (p: "-L native=${p}/lib") buildInputs)}"
          export LD_LIBRARY_PATH="${builtins.toString (pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs))}"

          export CMAKE_AR="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}ar"
          export CMAKE_C_COMPILER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc"
          export CMAKE_CXX_COMPILER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}c++"
          export CMAKE_RC_COMPILER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}windres"
        '';
      };
    });
  };
}
