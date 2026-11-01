{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    pkg-config
    glfw
    cmake
    clang
    wayland
    libx11
    libXrandr
    libXinerama
    libXcursor
    libXi
    libxkbcommon
    libGL
    # Web support (uncomment to enable) -- Untested - @JamesKEbert
    # emscripten
  ];

  LD_LIBRARY_PATH =
    with pkgs;
    lib.makeLibraryPath [
      libGL
      glfw
      wayland
      libx11
      libXrandr
      libXinerama
      libXcursor
      libXi
      libxkbcommon
    ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
