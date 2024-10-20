{
  description = "Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
  in
  {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = [ pkgs.cargo ];
      LD_LIBRARY_PATH = libPath;
    };
  };
}
