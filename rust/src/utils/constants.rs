pub const MESH_EPSILON: f64 = 1E-6;

pub const RAY_OFFSET_EPSILON: f64 = 1E-6;
pub const GAMMA_CORRECTION: f64 = 1. / 2.2;
pub const MAX_RECURSION_DEPTH: i32 = 4;
pub const FRESNEL_RAYS: i32 = 4096;
pub const INDIRECT_LIGHTING_RAYS: i32 = 256;
pub const ANTIALIASING_RAYS: i32 = 32;
pub const MIN_BVH_NODE_SIZE: usize = 5;

pub const ENABLE_FRESNEL: bool = true;
pub const ENABLE_INDIRECT_LIGHTING: bool = true;
pub const ENABLE_ANTIALIASING: bool = true;
