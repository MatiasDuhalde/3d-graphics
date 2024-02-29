use {
    crate::{
        core::{Intersectable, Intersection, Ray},
        utils::{calculate_rotation_matrix, Vector3, MESH_EPSILON},
    },
    std::fs,
};

#[derive(Clone)]
pub struct TriangleIndices {
    vertex_indices: (usize, usize, usize),
    normal_indices: (usize, usize, usize),
    uv_indices: (usize, usize, usize),
}

impl TriangleIndices {
    pub fn get_vertex_indices(&self) -> (usize, usize, usize) {
        self.vertex_indices
    }

    pub fn get_normal_indices(&self) -> (usize, usize, usize) {
        self.normal_indices
    }

    pub fn get_uv_indices(&self) -> (usize, usize, usize) {
        self.uv_indices
    }
}

#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<Vector3>,
    normals: Vec<Vector3>,
    uvs: Vec<Vector3>,
    triangles: Vec<TriangleIndices>,
}

impl Mesh {
    pub fn get_vertices(&self) -> &Vec<Vector3> {
        &self.vertices
    }

    pub fn get_triangles(&self) -> &Vec<TriangleIndices> {
        &self.triangles
    }

    pub fn get_triangle(&self, index: usize) -> &TriangleIndices {
        &self.triangles[index]
    }

    pub fn swap_triangles(&mut self, i: usize, j: usize) {
        self.triangles.swap(i, j);
    }
}

impl Mesh {
    pub fn translate(&mut self, translation: Vector3) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            *vertex += translation;
        }
        self
    }

    pub fn rotate(&mut self, rotation: Vector3) -> &mut Self {
        let rotation_matrix = calculate_rotation_matrix(rotation);
        for vertex in self.vertices.iter_mut() {
            *vertex = rotation_matrix * *vertex;
        }
        self
    }

    pub fn scale(&mut self, scale: f64) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            *vertex *= scale;
        }
        self
    }

    pub fn calculate_triangle_center(&self, triangle: &TriangleIndices) -> Vector3 {
        let vertex_indices = triangle.get_vertex_indices();
        let a = self.vertices[vertex_indices.0];
        let b = self.vertices[vertex_indices.1];
        let c = self.vertices[vertex_indices.2];
        (a + b + c) / 3.
    }

    pub fn intersect_part(
        &self,
        ray: &Ray,
        start_triangle_index: usize,
        end_triangle_index: usize,
    ) -> Option<Intersection> {
        let mut closest_distance = f64::INFINITY;
        let mut closest_point = Vector3::new(0., 0., 0.);
        let mut closest_normal = Vector3::new(0., 0., 0.);
        let mut closest_exterior = true;
        for triangle in &self.triangles[start_triangle_index..end_triangle_index] {
            let u = *ray.get_direction();
            let o = *ray.get_origin();

            let vertex_indices = triangle.get_vertex_indices();

            let a = self.vertices[vertex_indices.0];
            let b = self.vertices[vertex_indices.1];
            let c = self.vertices[vertex_indices.2];

            let e1 = b - a;
            let e2 = c - a;
            let n = e1.cross(&e2);
            let u_dot_n = u.dot(&n);
            if u_dot_n.abs() < MESH_EPSILON {
                continue;
            }

            let a_o = a - o;
            let a_o_x_u = a_o.cross(&u);

            let beta = e2.dot(&a_o_x_u) / u_dot_n;
            if beta < 0. || beta > 1. {
                continue;
            }

            let gamma = -e1.dot(&a_o_x_u) / u_dot_n;
            if gamma < 0. || gamma + beta > 1. {
                continue;
            }

            let t = a_o.dot(&n) / u_dot_n;

            if t > MESH_EPSILON && t < closest_distance {
                closest_distance = t;
                closest_point = o + u * t;
                closest_normal = n.normalized();
                closest_exterior = u_dot_n < 0.;
            }
        }

        if closest_distance == f64::INFINITY {
            None
        } else {
            Some(Intersection::new(
                closest_point,
                closest_normal,
                closest_distance,
                closest_exterior,
                None,
                ray.clone(),
            ))
        }
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.intersect_part(ray, 0, self.triangles.len())
    }
}

impl Mesh {
    pub fn from_obj_file(filename: &str) -> Mesh {
        let mut vertices: Vec<Vector3> = Vec::new();
        let mut normals: Vec<Vector3> = Vec::new();
        let mut uvs: Vec<Vector3> = Vec::new();
        let mut triangles: Vec<TriangleIndices> = Vec::new();

        let contents = fs::read_to_string(filename).expect("Failed to read file");
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 0 {
                continue;
            }
            match parts[0] {
                "v" => vertices.push(parse_vector_from_tokens(parts)),
                "vn" => normals.push(parse_vector_from_tokens(parts)),
                "vt" => uvs.push(parse_vector_from_tokens(parts)),
                "f" => {
                    let mut vertex_indices: [usize; 3] = [0, 0, 0];
                    let mut normal_indices: [usize; 3] = [0, 0, 0];
                    let mut uv_indices: [usize; 3] = [0, 0, 0];
                    for i in 0..3 {
                        let face_parts: Vec<&str> = parts[i + 1].split('/').collect();
                        vertex_indices[i] = face_parts[0].parse::<usize>().unwrap() - 1;
                        uv_indices[i] = face_parts[1].parse::<usize>().unwrap() - 1;
                        normal_indices[i] = face_parts[2].parse::<usize>().unwrap() - 1;
                    }
                    triangles.push(TriangleIndices {
                        vertex_indices: (vertex_indices[0], vertex_indices[1], vertex_indices[2]),
                        normal_indices: (normal_indices[0], normal_indices[1], normal_indices[2]),
                        uv_indices: (uv_indices[0], uv_indices[1], uv_indices[2]),
                    })
                }
                _ => {}
            }
        }

        Mesh {
            vertices,
            normals,
            uvs,
            triangles,
        }
    }
}

fn parse_vector_from_tokens(tokens: Vec<&str>) -> Vector3 {
    let x = tokens[1].parse::<f64>().unwrap();
    let y = tokens[2].parse::<f64>().unwrap();
    let z = tokens[3].parse::<f64>().unwrap();
    Vector3::new(x, y, z)
}
