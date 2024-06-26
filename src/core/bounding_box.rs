use crate::{
    core::{Intersectable, Intersection, Mesh, Ray},
    utils::Vector3,
};

pub struct BoundingBox {
    min: Vector3,
    max: Vector3,
}

impl BoundingBox {
    pub fn new_from_mesh(mesh: &Mesh) -> BoundingBox {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        for vertex in mesh.get_vertices() {
            if vertex.x() < min_x {
                min_x = vertex.x();
            }
            if vertex.y() < min_y {
                min_y = vertex.y();
            }
            if vertex.z() < min_z {
                min_z = vertex.z();
            }
            if vertex.x() > max_x {
                max_x = vertex.x();
            }
            if vertex.y() > max_y {
                max_y = vertex.y();
            }
            if vertex.z() > max_z {
                max_z = vertex.z();
            }
        }

        BoundingBox {
            min: Vector3::new(min_x, min_y, min_z),
            max: Vector3::new(max_x, max_y, max_z),
        }
    }

    pub fn new_from_mesh_and_triangle_indices(
        mesh: &Mesh,
        start_triangle_index: usize,
        end_triangle_index: usize,
    ) -> BoundingBox {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        let triangles_slice = &mesh.get_triangles()[start_triangle_index..end_triangle_index];

        for triangle in triangles_slice {
            let vertex_indices = triangle.get_vertex_indices();

            for vertex_index in [vertex_indices.0, vertex_indices.1, vertex_indices.2] {
                let vertex = mesh.get_vertices()[vertex_index];

                if vertex.x() < min_x {
                    min_x = vertex.x();
                }
                if vertex.y() < min_y {
                    min_y = vertex.y();
                }
                if vertex.z() < min_z {
                    min_z = vertex.z();
                }
                if vertex.x() > max_x {
                    max_x = vertex.x();
                }
                if vertex.y() > max_y {
                    max_y = vertex.y();
                }
                if vertex.z() > max_z {
                    max_z = vertex.z();
                }
            }
        }

        BoundingBox {
            min: Vector3::new(min_x, min_y, min_z),
            max: Vector3::new(max_x, max_y, max_z),
        }
    }

    /// Return the length of each diagonal
    pub fn calculate_diagonals(&self) -> Vector3 {
        self.max - self.min
    }

    pub fn calculate_center(&self) -> Vector3 {
        (self.min + self.max) / 2.
    }
}

impl Intersectable for BoundingBox {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let point_a = self.min;
        let point_b = Vector3::new(self.max.x(), self.min.y(), self.min.z());
        let point_c = Vector3::new(self.min.x(), self.min.y(), self.max.z());
        let point_d = Vector3::new(self.max.x(), self.min.y(), self.max.z());
        let point_e = Vector3::new(self.min.x(), self.max.y(), self.min.z());
        let point_f = Vector3::new(self.max.x(), self.max.y(), self.min.z());
        let point_g = Vector3::new(self.min.x(), self.max.y(), self.max.z());
        let point_h = self.max;

        let origin = *ray.get_origin();
        let direction = *ray.get_direction();

        let normal_1 = (point_b - point_a).cross(&(point_c - point_a));
        let direction_dot_normal_1 = direction.dot(&normal_1);
        let t_1 = (point_a - origin).dot(&normal_1) / direction_dot_normal_1;

        if t_1 > 0. {
            let intersection_point_1 = origin + direction * t_1;

            if (intersection_point_1.x() > point_a.x() && intersection_point_1.x() < point_d.x())
                && (intersection_point_1.z() > point_a.z()
                    && intersection_point_1.z() < point_d.z())
            {
                return Some(Intersection::new(
                    intersection_point_1,
                    normal_1,
                    t_1,
                    direction_dot_normal_1 < 0.,
                    None,
                    None,
                    ray.clone(),
                ));
            }
        }

        let normal_2 = -normal_1;
        let direction_dot_normal_2 = direction.dot(&normal_2);
        let t_2 = (point_e - origin).dot(&normal_2) / direction_dot_normal_2;

        if t_2 > 0. {
            let intersection_point_2 = origin + direction * t_2;

            if (intersection_point_2.x() > point_e.x() && intersection_point_2.x() < point_h.x())
                && (intersection_point_2.z() > point_e.z()
                    && intersection_point_2.z() < point_h.z())
            {
                return Some(Intersection::new(
                    intersection_point_2,
                    normal_2,
                    t_2,
                    direction_dot_normal_2 < 0.,
                    None,
                    None,
                    ray.clone(),
                ));
            }
        }

        let normal_3 = (point_c - point_a).cross(&(point_e - point_a));
        let direction_dot_normal_3 = direction.dot(&normal_3);
        let t_3 = (point_a - origin).dot(&normal_3) / direction_dot_normal_3;

        if t_3 > 0. {
            let intersection_point_3 = origin + direction * t_3;

            if (intersection_point_3.y() > point_a.y() && intersection_point_3.y() < point_g.y())
                && (intersection_point_3.z() > point_a.z()
                    && intersection_point_3.z() < point_g.z())
            {
                return Some(Intersection::new(
                    intersection_point_3,
                    normal_3,
                    t_3,
                    direction_dot_normal_3 < 0.,
                    None,
                    None,
                    ray.clone(),
                ));
            }
        }

        let normal_4 = -normal_3;
        let direction_dot_normal_4 = direction.dot(&normal_4);
        let t_4 = (point_b - origin).dot(&normal_4) / direction_dot_normal_4;

        if t_4 > 0. {
            let intersection_point_4 = origin + direction * t_4;

            if (intersection_point_4.y() > point_b.y() && intersection_point_4.y() < point_h.y())
                && (intersection_point_4.z() > point_b.z()
                    && intersection_point_4.z() < point_h.z())
            {
                return Some(Intersection::new(
                    intersection_point_4,
                    normal_4,
                    t_4,
                    direction_dot_normal_4 < 0.,
                    None,
                    None,
                    ray.clone(),
                ));
            }
        }

        let normal_5 = (point_e - point_a).cross(&(point_b - point_a));
        let direction_dot_normal_5 = direction.dot(&normal_5);
        let t_5 = (point_a - origin).dot(&normal_5) / direction_dot_normal_5;

        if t_5 > 0. {
            let intersection_point_5 = origin + direction * t_5;

            if (intersection_point_5.x() > point_a.x() && intersection_point_5.x() < point_f.x())
                && (intersection_point_5.y() > point_a.y()
                    && intersection_point_5.y() < point_f.y())
            {
                return Some(Intersection::new(
                    intersection_point_5,
                    normal_5,
                    t_5,
                    direction_dot_normal_5 < 0.,
                    None,
                    None,
                    ray.clone(),
                ));
            }
        }

        let normal_6 = -normal_5;
        let direction_dot_normal_6 = direction.dot(&normal_6);
        let t_6 = (point_c - origin).dot(&normal_6) / direction_dot_normal_6;

        if t_6 > 0. {
            let intersection_point_6 = origin + direction * t_6;

            if (intersection_point_6.x() > point_c.x() && intersection_point_6.x() < point_h.x())
                && (intersection_point_6.y() > point_c.y()
                    && intersection_point_6.y() < point_h.y())
            {
                return Some(Intersection::new(
                    intersection_point_6,
                    normal_6,
                    t_6,
                    direction_dot_normal_6 < 0.,
                    None,
                    None,
                    ray.clone(),
                ));
            }
        }

        None
    }
}
