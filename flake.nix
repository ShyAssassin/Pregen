{
  description = "The Pregen game engine";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

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
            libGL
            gdb valgrind
            rustup mold unzip
            vulkan-headers vulkan-loader
            vulkan-tools vulkan-tools-lunarg
            pkg-config cmake extra-cmake-modules
            vulkan-extension-layer vulkan-validation-layers
            wayland wayland-protocols wayland-scanner libxkbcommon
            xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi xorg.libXinerama
          ];

          shellHook = ''
            rustup default stable
            rustup component add rust-src rust-analyzer
            # Add build inputs to the LD_LIBRARY_PATH
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          '';
        };
      };
    };
}
