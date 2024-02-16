use crate::{
    core::{Intersectable, Intersection, IntersectionBuilder, Ray},
    utils::{Mesh, Vector3},
};

const DEFAULT_ALBEDO: Vector3 = Vector3::new(1., 1., 1.);

pub struct MeshObject {
    mesh: Mesh,
    albedo: Vector3,
}

pub struct MeshObjectBuilder {
    mesh: Mesh,
    albedo: Vector3,
}

impl MeshObjectBuilder {
    pub fn new(mesh: &Mesh) -> Self {
        MeshObjectBuilder {
            mesh: mesh.clone(),
            albedo: DEFAULT_ALBEDO,
        }
    }

    pub fn with_albedo(&mut self, albedo: Vector3) -> &mut Self {
        self.albedo = albedo;
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
            albedo: self.albedo,
        }
    }
}

impl Intersectable for MeshObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;
        for triangle in self.mesh.get_triangles() {
            let v0 = self.mesh.get_vertices()[triangle.vertex_indices[0]];
            let v1 = self.mesh.get_vertices()[triangle.vertex_indices[1]];
            let v2 = self.mesh.get_vertices()[triangle.vertex_indices[2]];

            let e1 = v1 - v0;
            let e2 = v2 - v0;
            let h = ray.get_direction().cross(&e2);
            let a = e1.dot(&h);
            if a > -0.00001 && a < 0.00001 {
                continue;
            }
            let f = 1.0 / a;
            let s = *ray.get_origin() - v0;
            let u = f * s.dot(&h);
            if u < 0.0 || u > 1.0 {
                continue;
            }
            let q = s.cross(&e1);
            let v = f * ray.get_direction().dot(&q);
            if v < 0.0 || u + v > 1.0 {
                continue;
            }
            let t = f * e2.dot(&q);
            if t > 0.00001 {
                let intersection_point = *ray.get_origin() + *ray.get_direction() * t;
                let normal = (e1.cross(&e2)).normalize();
                let mut builder = IntersectionBuilder::new(intersection_point, normal, t);
                builder.with_albedo(self.albedo);
                let intersection = builder.build();

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
