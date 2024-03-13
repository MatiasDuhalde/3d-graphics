# Rust ray tracer

This repository implements a simple, CPU-bound ray tracer using Rust.

All calculations are done on the CPU. These can be particularly heavy for specific parameters and scene configurations.

The following libraries are used:

- `image` for image output
- `rayon` for parallelism
- `rand` for random number generation

## Usage

To run the package, use:

```bash
cargo run --release
```

The scene can be modified in the `src/main.rs` file.

This will output an image file (name can be modified in the `src/main.rs` file) in the root directory.

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
