# Pachinkox

Tanner Weber 2026

Basic simulation of a pachinko machine in rust.

## Build and Run

[Raylib](https://github.com/raylib-rs/raylib-rs) dependencies need to be on
your system for the binary crate.
I've only tested on Wayland on NixOS.

Dependencies for Debian/Ubuntu (from the raylib-rs README):

```
cmake libasound2-dev libudev-dev libx11-dev libxrandr-dev libxinerama-dev
libxcursor-dev libxi-dev libgl1-mesa-dev
libglfw3-dev wayland-devel libxkbcommon-devel wayland-protocols
wayland-protocols-devel libecm-dev
```

For NixOS, there is a `shell.nix`. `nix-shell ./shell.nix`

To build:

```sh
cargo build
```

To run:

```sh
cargo run
```

Other checks:

```sh
cargo fmt && cargo clippy && cargo test
```

## Writeup

The library crate contains a 'World' struct that contains a bunch of entities
that have physics processed each tick. The entity types are enum variants.
The entities in the library crate are just basic shapes of circle and lines.
The physics are very primitive, and there may be issues with phasing through
at high speeds or gettings stuck in pins at very low speeds or boring behavior
sometimes.

The frontend is made using [raylib](https://www.raylib.com/).

## License

Copyright (C) 2026 Tanner Weber

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
