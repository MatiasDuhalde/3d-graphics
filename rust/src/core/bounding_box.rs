use crate::{
    core::{Intersectable, Intersection, IntersectionBuilder, Ray},
    utils::{Mesh, Vector3},
};

pub struct BoundingBox {
    min: Vector3,
    max: Vector3,
}

impl BoundingBox {
    pub fn get_min(&self) -> &Vector3 {
        &self.min
    }

    pub fn get_max(&self) -> &Vector3 {
        &self.max
    }

    pub fn new(mesh: &Mesh) -> BoundingBox {
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
}

impl Intersectable for BoundingBox {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let u = *ray.get_direction();
        let o = *ray.get_origin();

        // test bounding box
        let t_min = (self.get_min().x() - o.x()) / u.x();
        let t_max = (self.get_max().x() - o.x()) / u.x();

        let (t_min, t_max) = if t_min > t_max {
            (t_max, t_min)
        } else {
            (t_min, t_max)
        };

        let t_y_min = (self.get_min().y() - o.y()) / u.y();
        let t_y_max = (self.get_max().y() - o.y()) / u.y();

        let (t_y_min, t_y_max) = if t_y_min > t_y_max {
            (t_y_max, t_y_min)
        } else {
            (t_y_min, t_y_max)
        };

        if t_min > t_y_max || t_y_min > t_max {
            return None;
        }

        let t_min = if t_y_min > t_min { t_y_min } else { t_min };
        let t_max = if t_y_max < t_max { t_y_max } else { t_max };

        let t_z_min = (self.get_min().z() - o.z()) / u.z();
        let t_z_max = (self.get_max().z() - o.z()) / u.z();

        let (t_z_min, t_z_max) = if t_z_min > t_z_max {
            (t_z_max, t_z_min)
        } else {
            (t_z_min, t_z_max)
        };

        if t_min > t_z_max || t_z_min > t_max {
            return None;
        }

        let t_min = if t_z_min > t_min { t_z_min } else { t_min };
        let t_max = if t_z_max < t_max { t_z_max } else { t_max };

        if t_min < 0. {
            if t_max < 0. {
                return None;
            }
        }

        let intersection_point = o + u * t_min;

        let builder = IntersectionBuilder::new(intersection_point, normal, t);

        Some(builder.build())
    }
}
