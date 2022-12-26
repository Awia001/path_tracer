# Rust Pathtracer, writen in rust
A very basic pathtracer written in rust which renders using SDL2, "realtime" in the sense that it just draws whatever it has every frame and doesn't care.
You must be running Rust Nightly to get access to portable simd in the standard library, I don't make the rules.

Uses `vcpkg` to manage building library dependencies, make sure you have `cargo-vcpkg` installed and run `cargo vcpkg build` before running `cargo build`

To run it, just run it, you'll get what you're given.

## Todo
- Refactor renderer into own module
- Shading
- Normals
- Actually bounce rays multiple times
- Gamma correction
- Probably lots more...

## Possibles?
- Maybe rasterise a single frame and do edge detection on the resulting image so that rays can be cast at interesting parts of the image first?
- AABB and BVH structures so we can go fast?

