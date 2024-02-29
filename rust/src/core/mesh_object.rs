use crate::{
    core::{BoundingBox, Intersectable, Intersection, Mesh, Object, Ray},
    utils::Vector3,
};

const DEFAULT_OPAQUE: bool = false;
const DEFAULT_COLOR: Vector3 = Vector3::new(1., 1., 1.);
const DEFAULT_MIRROR: bool = false;
const DEFAULT_TRANSPARENT: bool = false;
const DEFAULT_REFRACTIVE_INDEX: f64 = 1.;

pub struct MeshObject {
    mesh: Mesh,
    opaque: bool,
    color: Vector3,
    mirror: bool,
    transparent: bool,
    refractive_index: f64,
    bounding_box: BoundingBox,
}

pub struct MeshObjectBuilder {
    mesh: Mesh,
    opaque: bool,
    color: Vector3,
    mirror: bool,
    transparent: bool,
    refractive_index: f64,
}

impl MeshObjectBuilder {
    pub fn new(mesh: &Mesh) -> Self {
        MeshObjectBuilder {
            mesh: mesh.clone(),
            opaque: DEFAULT_OPAQUE,
            color: DEFAULT_COLOR,
            mirror: DEFAULT_MIRROR,
            transparent: DEFAULT_TRANSPARENT,
            refractive_index: DEFAULT_REFRACTIVE_INDEX,
        }
    }

    pub fn with_scale(&mut self, scale: f64) -> &mut Self {
        self.mesh.scale(scale);
        self
    }

    pub fn with_translation(&mut self, translation: Vector3) -> &mut Self {
        self.mesh.translate(translation);
        self
    }

    pub fn with_rotation(&mut self, rotation: Vector3) -> &mut Self {
        self.mesh.rotate(rotation);
        self
    }

    pub fn with_opaque(&mut self, opaque: bool) -> &mut Self {
        self.opaque = opaque;
        self
    }

    pub fn with_color(&mut self, color: Vector3) -> &mut Self {
        self.opaque = true;
        self.color = color;
        self
    }

    pub fn with_mirror(&mut self, mirror: bool) -> &mut Self {
        self.mirror = mirror;
        self
    }

    pub fn with_transparent(&mut self, transparent: bool) -> &mut Self {
        self.transparent = transparent;
        self
    }

    pub fn with_refractive_index(&mut self, refractive_index: f64) -> &mut Self {
        self.transparent = true;
        self.refractive_index = refractive_index;
        self
    }

    pub fn build(&self) -> MeshObject {
        MeshObject {
            mesh: self.mesh.clone(),
            opaque: self.opaque,
            color: self.color,
            mirror: self.mirror,
            transparent: self.transparent,
            refractive_index: self.refractive_index,
            bounding_box: BoundingBox::new_from_mesh(&self.mesh),
        }
    }
}

impl Intersectable for MeshObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bounding_box.intersect(ray).is_none() {
            return None;
        }
        let intersection = self.mesh.intersect(ray);

        intersection.and_then(|mut i| {
            i.set_object(self);
            Some(i)
        })
    }
}

impl Object for MeshObject {
    fn is_opaque(&self) -> bool {
        self.opaque
    }

    fn is_mirror(&self) -> bool {
        self.mirror
    }

    fn is_transparent(&self) -> bool {
        self.transparent
    }

    fn is_light_source(&self) -> bool {
        false
    }

    fn get_color(&self) -> &Vector3 {
        &self.color
    }

    fn get_refractive_index(&self) -> f64 {
        self.refractive_index
    }

    fn get_light_intensity(&self) -> f64 {
        0.
    }
}
