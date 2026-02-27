{ pkgs ? import <nixpkgs> {
    overlays = [ (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ];
  }
}:
let
  dlopenLibs = with pkgs; [
    # XKB common library
    libxkbcommon
    # GPU backend
    vulkan-loader
    # Window system
    wayland
  ];
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    rust-bin.stable.latest.default
  ];
  buildInputs = dlopenLibs;
  env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibs}";
  shellHook = ''
    echo "Rust env up w/ oxalica rust-overlay"
    echo "RUSTFLAGS: $RUSTFLAGS"
    rustc --version
  '';
}
