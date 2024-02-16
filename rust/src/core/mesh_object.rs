use crate::{
    core::{BoundingBox, Intersectable, Intersection, IntersectionBuilder, Ray},
    utils::{Mesh, Vector3, MESH_EPSILON},
};

const DEFAULT_ALBEDO: Vector3 = Vector3::new(1., 1., 1.);
const DEFAULT_MIRROR: bool = false;

pub struct MeshObject {
    mesh: Mesh,
    color: Vector3,
    mirror: bool,
    bounding_box: BoundingBox,
}

pub struct MeshObjectBuilder {
    mesh: Mesh,
    albedo: Vector3,
    mirror: bool,
}

impl MeshObjectBuilder {
    pub fn new(mesh: &Mesh) -> Self {
        MeshObjectBuilder {
            mesh: mesh.clone(),
            albedo: DEFAULT_ALBEDO,
            mirror: DEFAULT_MIRROR,
        }
    }

    pub fn with_color(&mut self, albedo: Vector3) -> &mut Self {
        self.albedo = albedo;
        self
    }

    pub fn with_mirror(&mut self, mirror: bool) -> &mut Self {
        self.mirror = mirror;
        self
    }

    pub fn with_rotation(&mut self, rotation: Vector3) -> &mut Self {
        self.mesh.rotate(rotation);
        self
    }

    pub fn with_scale(&mut self, scale: f64) -> &mut Self {
        self.mesh.scale(scale);
        self
    }

    pub fn with_translation(&mut self, translation: Vector3) -> &mut Self {
        self.mesh.translate(translation);
        self
    }

    pub fn build(&self) -> MeshObject {
        MeshObject {
            mesh: self.mesh.clone(),
            color: self.albedo,
            mirror: self.mirror,
            bounding_box: BoundingBox::new(&self.mesh),
        }
    }
}

impl Intersectable for MeshObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bounding_box.intersect(ray).is_none() {
            return None;
        }

        let u = *ray.get_direction();
        let o = *ray.get_origin();

        let mut closest_intersection: Option<Intersection> = None;
        for triangle in self.mesh.get_triangles() {
            let a = self.mesh.get_vertices()[triangle.vertex_indices[0]];
            let b = self.mesh.get_vertices()[triangle.vertex_indices[1]];
            let c = self.mesh.get_vertices()[triangle.vertex_indices[2]];

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

            let t = a_o.dot(&n) / u.dot(&n);

            if t > MESH_EPSILON {
                let p = o + u * t;
                let normal = n.normalized();

                let mut intersection_builder = IntersectionBuilder::new(p, normal, t);

                if self.mirror {
                    let reflected_ray = ray.calculate_reflected_ray(&p, &normal).add_offset();
                    intersection_builder.with_reflected_ray(reflected_ray);
                } else {
                    intersection_builder.with_albedo(self.color);
                }

                let intersection = intersection_builder.build();

                if closest_intersection.is_some() {
                    if t < closest_intersection.as_ref().unwrap().get_distance() {
                        closest_intersection = Some(intersection);
                    }
                } else {
                    closest_intersection = Some(intersection);
                }
            }
        }
        closest_intersection
    }
}
