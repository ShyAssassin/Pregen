{
  description = "The Pregen game engine";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells = {
        x86_64-linux.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            gdb valgrind

            # Build dependencies
            rustup mold unzip
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
            rustup component add rust-src rust-analyzer
            cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer --rev v0.8.1 wgsl_analyzer
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          '';
        };
      };
    };
}
