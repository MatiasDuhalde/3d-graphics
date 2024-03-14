use {
    crate::{
        core::{Intersectable, Intersection, Ray},
        utils::{calculate_rotation_matrix, Vector3, ENABLE_NORMAL_MAPPING, MESH_EPSILON},
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
        for normal in self.normals.iter_mut() {
            *normal = rotation_matrix * *normal;
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
        let mut closest_normal = Vector3::new(0., 0., 0.);
        let mut closest_triangle = &self.triangles[0];
        let mut closest_alpha = 0.;
        let mut closest_beta = 0.;
        let mut closest_gamma = 0.;
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
                closest_triangle = triangle;
                closest_alpha = 1. - beta - gamma;
                closest_beta = beta;
                closest_gamma = gamma;
                closest_exterior = u_dot_n < 0.;
                closest_normal = n;
            }
        }

        if closest_distance == f64::INFINITY {
            None
        } else {
            let distance = closest_distance;
            let point = *ray.get_origin() + *ray.get_direction() * distance;
            let normal = if ENABLE_NORMAL_MAPPING {
                let normal_indices = closest_triangle.get_normal_indices();

                let normal_a = self.normals[normal_indices.0];
                let normal_b = self.normals[normal_indices.1];
                let normal_c = self.normals[normal_indices.2];

                let shading_normal =
                    closest_alpha * normal_a + closest_beta * normal_b + closest_gamma * normal_c;

                shading_normal.normalized()
            } else {
                closest_normal.normalized()
            };

            let uv_indices = closest_triangle.get_uv_indices();
            let uv_a = self.uvs[uv_indices.0];
            let uv_b = self.uvs[uv_indices.1];
            let uv_c = self.uvs[uv_indices.2];

            let mapping_point = closest_alpha * uv_a + closest_beta * uv_b + closest_gamma * uv_c;
            let map_x = (mapping_point.x() % 1.).abs();
            let map_y = (mapping_point.y() % 1.).abs();
            let map_point = Vector3::new(map_x, map_y, 0.);

            Some(Intersection::new(
                point,
                normal,
                distance,
                closest_exterior,
                Some(map_point),
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
                "v" => vertices.push(Self::parse_vertex(&parts)),
                "vn" => normals.push(Self::parse_normal(&parts)),
                "vt" => uvs.push(Self::parse_uv(&parts)),
                "vp" => {} // vp is not supported
                "f" => triangles.push(Self::parse_face(&parts)),
                "mtllib" => {} // mtllib is managed elsewhere
                "usemtl" => {} // usemtl is managed elsewhere
                "s" => {}      // s is managed globally
                "g" => {}      // g is not supported
                "#" => {}      // comment
                _ => {
                    println!("Unknown line: {}", line);
                }
            }
        }

        Mesh {
            vertices,
            normals,
            uvs,
            triangles,
        }
    }

    fn parse_vertex(tokens: &Vec<&str>) -> Vector3 {
        if tokens.len() < 4 {
            panic!("Invalid vertex line: {:?}", tokens);
        }
        let x = tokens[1].parse::<f64>().unwrap();
        let y = tokens[2].parse::<f64>().unwrap();
        let z = tokens[3].parse::<f64>().unwrap();

        // the w component is optional, and not used in this context
        let _w = if tokens.len() > 4 {
            tokens[4].parse::<f64>().unwrap()
        } else {
            1.
        };
        Vector3::new(x, y, z)
    }

    fn parse_normal(tokens: &Vec<&str>) -> Vector3 {
        if tokens.len() < 4 {
            panic!("Invalid normal line: {:?}", tokens);
        }
        let x = tokens[1].parse::<f64>().unwrap();
        let y = tokens[2].parse::<f64>().unwrap();
        let z = tokens[3].parse::<f64>().unwrap();
        Vector3::new(x, y, z).normalized()
    }

    fn parse_uv(tokens: &Vec<&str>) -> Vector3 {
        if tokens.len() < 2 {
            panic!("Invalid uv line: {:?}", tokens);
        }
        let u = tokens[1].parse::<f64>().unwrap();
        let v = if tokens.len() > 2 {
            tokens[2].parse::<f64>().unwrap()
        } else {
            0.
        };
        let _w = if tokens.len() > 3 {
            tokens[3].parse::<f64>().unwrap()
        } else {
            0.
        };
        Vector3::new(u, v, 0.)
    }

    fn parse_face(tokens: &Vec<&str>) -> TriangleIndices {
        if tokens.len() < 4 {
            panic!("Invalid face line: {:?}", tokens);
        }
        let mut vertex_indices: [usize; 3] = [0, 0, 0];
        let mut normal_indices: [usize; 3] = [0, 0, 0];
        let mut uv_indices: [usize; 3] = [0, 0, 0];
        for i in 0..3 {
            let face_parts: Vec<&str> = tokens[i + 1].split('/').collect();
            vertex_indices[i] = face_parts[0].parse::<usize>().unwrap() - 1;
            uv_indices[i] = face_parts[1].parse::<usize>().unwrap() - 1;
            normal_indices[i] = face_parts[2].parse::<usize>().unwrap() - 1;
        }
        TriangleIndices {
            vertex_indices: (vertex_indices[0], vertex_indices[1], vertex_indices[2]),
            normal_indices: (normal_indices[0], normal_indices[1], normal_indices[2]),
            uv_indices: (uv_indices[0], uv_indices[1], uv_indices[2]),
        }
    }
}
