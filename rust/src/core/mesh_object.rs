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

            let a_o = a - o;

            let t = a_o.dot(&n) / u.dot(&n);

            if t > 0. {
                let p = o + u * t;
                let normal = n.normalized();

                let intersection = IntersectionBuilder::new(p, normal, t)
                    .with_albedo(self.albedo)
                    .build();

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
