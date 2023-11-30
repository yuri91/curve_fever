{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.11";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, ... } @ inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default ];
    };
    rust-build = pkgs.rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
      targets = [];
    };
    naersk-lib = naersk.lib.${system}.override {
      rustc = rust-build;
      cargo = rust-build;
    };
    LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs; lib.makeLibraryPath [
      udev
      alsa-lib
      vulkan-loader
      libglvnd
      libxkbcommon
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      wayland
    ]}";
    curve_fever = naersk-lib.buildPackage {
      pname = "curve_fever";
      root = ./.;
      buildInputs = with pkgs; [
        alsa-lib
        libxkbcommon
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        libglvnd
        wayland
        vulkan-loader
        udev
      ];
      nativeBuildInputs = with pkgs; [
        pkg-config
        clang
        llvmPackages.bintools
        makeWrapper
        rust-build
      ];
      postInstall = ''
        wrapProgram $out/bin/curve_fever \
          --set LD_LIBRARY_PATH ${LD_LIBRARY_PATH}
      '';
    };
  in
  {
    devShell.${system} = pkgs.mkShell {
      packages = with pkgs; [
        git
        cargo-edit
        rust-analyzer-unwrapped
      ];
      inputsFrom = with pkgs; [
        curve_fever
      ];
      RUST_SRC_PATH = "${rust-build}/lib/rustlib/src/rust/library";
      inherit LD_LIBRARY_PATH;
    };
    packages.${system}.default = curve_fever;
  };
}
