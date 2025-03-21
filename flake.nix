{
  description = "The Pregen game engine";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    devShells = {
      # God is dead and we have killed him
      x86_64-linux.cross-x86_64-windows = pkgs.pkgsCross.mingwW64.mkShell rec {
        packages = with pkgs; [
          wineWowPackages.stable
          pkgs.pkgsCross.mingwW64.stdenv.cc
          pkgsCross.mingwW64.windows.pthreads
          rustup unzip cmake extra-cmake-modules
        ];

        shellHook = ''
          rustup default stable
          rustup target add x86_64-pc-windows-gnu
          export CARGO_BUILD_TARGET="x86_64-pc-windows-gnu"
          rustup component add rust-std rust-src rust-analyzer
          # GOD ONLY KNOWS WHY THIS IS NECESSARY BUT IT IS AND I HATE IT
          export RUSTFLAGS="-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath packages)}";
        '';
      };

      x86_64-linux.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          lldb valgrind

          # Build dependencies
          rustup mold unzip emscripten
          pkg-config cmake extra-cmake-modules

          # Vulkan
          vulkan-headers vulkan-loader
          vulkan-tools vulkan-tools-lunarg
          vulkan-extension-layer vulkan-validation-layers

          # Wayland
          wayland wayland-protocols wayland-scanner libxkbcommon

          # X11
          xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi xorg.libXinerama
        ];

        shellHook = ''
          rustup default stable
          export EM_CACHE=~/.emscripten_cache
          rustup target add wasm32-unknown-unknown
          rustup target add wasm32-unknown-emscripten
          rustup component add rust-std rust-src rust-analyzer
          cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer --rev v0.9.5 wgsl_analyzer
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
        '';
      };
    };
  };
}
