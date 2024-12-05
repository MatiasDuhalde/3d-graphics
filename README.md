# Rust ray tracer

This repository implements a simple, CPU-bound ray tracer using Rust.

All calculations are done on the CPU. These can be particularly heavy for specific parameters and scene configurations.

The following libraries are used:

- `image` for image output
- `rayon` for parallelism
- `rand` for random number generation

## Usage

Some sample assets used in the main program are included in this repository, in the `assets` directory. These are tracked through Git LFS, which needs to be installed in order to pull the files (see <https://git-lfs.com/>).

To compile and run the ray tracer, one must have Rust and Cargo installed. This can be very easily done by following the instructions in the official Rust website: <https://doc.rust-lang.org/cargo/getting-started/installation.html>

The ray tracer can be compiled with the following command:

```bash
cargo build --release
```

Note that some warnings may appear, all of which are related to unused methods or dead code. These can be safely ignored.

This will build an optimized binary in the `target/release` directory. The ray tracer can then be run with the following command:

```bash
# either
cargo run --release
# or
./target/release/raytracing-rust
```

This will render a scene and save the result to an image file.

Currently, there is no way of describing the scene in a separate file, so the scene is hard-coded in the `main.rs` file. This file contains several functions that define different scenes.

The parameters of the scene can be modified in the `src/utils/constants.rs` file. The following parameters can be modified.

- `MESH_EPSILON`: The epsilon value used for mesh intersection.
- `RAY_OFFSET_EPSILON`: The epsilon value used to offset reflected rays from the surface, to avoid self-intersection.
- `GAMMA_CORRECTION`: The gamma correction value used for the image output.
- `MAX_RECURSION_DEPTH`: The maximum recursion depth for the ray tracer.
- `FRESNEL_RAYS`: The amount of rays used for the Fresnel effect.
- `INDIRECT_LIGHTING_RAYS`: The amount of rays used for indirect lighting.
- `ANTIALIASING_RAYS`: The amount of rays used for anti-aliasing.
- `MIN_BVH_NODE_SIZE`: The amount of triangles in a BVH node before it is stopped from splitting.
- `ENABLE_FRESNEL`: Whether to enable the Fresnel effect.
- `ENABLE_INDIRECT_LIGHTING`: Whether to enable indirect lighting.
- `ENABLE_ANTIALIASING`: Whether to enable anti-aliasing.
- `ENABLE_NORMAL_MAPPING`: Whether to enable normal mapping to mesh objects.
