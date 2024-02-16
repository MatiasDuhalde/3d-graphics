use {crate::utils::Vector3, std::fs};

pub struct TriangleIndices {
    pub vertex_indices: [usize; 3],
    pub normal_indices: [usize; 3],
    pub uv_indices: [usize; 3],
}

pub struct Mesh {
    vertices: Vec<Vector3>,
    normals: Vec<Vector3>,
    uvs: Vec<Vector3>,
    triangles: Vec<TriangleIndices>,
}

impl Mesh {
    pub fn read_obj_file(filename: &str) -> Mesh {
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
                "v" => {
                    let x = parts[1].parse::<f64>().unwrap();
                    let y = parts[2].parse::<f64>().unwrap();
                    let z = parts[3].parse::<f64>().unwrap();
                    vertices.push(Vector3::new(x, y, z));
                }
                "vn" => {
                    let x = parts[1].parse::<f64>().unwrap();
                    let y = parts[2].parse::<f64>().unwrap();
                    let z = parts[3].parse::<f64>().unwrap();
                    normals.push(Vector3::new(x, y, z));
                }
                "vt" => {
                    let x = parts[1].parse::<f64>().unwrap();
                    let y = parts[2].parse::<f64>().unwrap();
                    let z = parts[3].parse::<f64>().unwrap();
                    uvs.push(Vector3::new(x, y, z));
                }
                "f" => {
                    let mut vertex_indices = [0; 3];
                    let mut normal_indices = [0; 3];
                    let mut uv_indices = [0; 3];
                    for i in 1..4 {
                        let face_parts: Vec<&str> = parts[i].split('/').collect();
                        vertex_indices[i - 1] = face_parts[0].parse::<usize>().unwrap() - 1;
                        uv_indices[i - 1] = face_parts[1].parse::<usize>().unwrap() - 1;
                        normal_indices[i - 1] = face_parts[2].parse::<usize>().unwrap() - 1;
                    }
                    triangles.push(TriangleIndices {
                        vertex_indices,
                        normal_indices,
                        uv_indices,
                    });
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
