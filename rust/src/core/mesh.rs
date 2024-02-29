use {
    crate::{
        core::{Intersectable, Intersection, Ray},
        utils::{calculate_rotation_matrix, Vector3, MESH_EPSILON},
    },
    std::fs,
};

#[derive(Clone)]
pub struct TriangleIndices {
    vertex_indices: [usize; 3],
    normal_indices: [usize; 3],
    uv_indices: [usize; 3],
}

impl TriangleIndices {
    pub fn get_vertex_indices(&self) -> [usize; 3] {
        self.vertex_indices
    }

    pub fn get_normal_indices(&self) -> [usize; 3] {
        self.normal_indices
    }

    pub fn get_uv_indices(&self) -> [usize; 3] {
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
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;
        for triangle in self.triangles.iter() {
            // let intersection = triangle.intersect(ray);

            let u = *ray.get_direction();
            let o = *ray.get_origin();

            let a = self.vertices[triangle.get_vertex_indices()[0]];
            let b = self.vertices[triangle.get_vertex_indices()[1]];
            let c = self.vertices[triangle.get_vertex_indices()[2]];

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

            let intersection = if t > MESH_EPSILON {
                let p = o + u * t;
                let normal = n.normalized();

                Some(Intersection::new(
                    p,
                    normal,
                    t,
                    u_dot_n < 0.,
                    None,
                    ray.clone(),
                ))
            } else {
                None
            };

            if intersection.is_some() {
                let intersection_reference = intersection.as_ref().unwrap();
                if closest_intersection.is_none()
                    || intersection_reference.get_distance()
                        < closest_intersection.as_ref().unwrap().get_distance()
                {
                    closest_intersection = intersection;
                }
            }
        }
        closest_intersection
    }
}

// impl Intersectable for TriangleIndices {
//     fn intersect(&self, ray: &Ray) -> Option<Intersection> {
//     }
// }

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
                        vertex_indices: [vertex_indices[0], vertex_indices[1], vertex_indices[2]],
                        normal_indices: [normal_indices[0], normal_indices[1], normal_indices[2]],
                        uv_indices: [uv_indices[0], uv_indices[1], uv_indices[2]],
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
