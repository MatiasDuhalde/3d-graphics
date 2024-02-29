use crate::{
    core::{BoundingBox, Intersectable, Intersection, Mesh, Ray},
    utils::MIN_BVH_NODE_SIZE,
};

pub struct BVHTree {
    root: BVHNode,
    mesh: Mesh,
}

pub struct BVHNode {
    bounding_box: BoundingBox,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    start_triangle_index: usize,
    end_triangle_index: usize,
}

impl BVHTree {
    pub fn new_from_mesh(mut mesh: Mesh) -> Self {
        let end = mesh.get_triangles().len();
        let root = BVHNode::new_from_mesh(&mut mesh, 0, end);
        BVHTree { root, mesh }
    }
}

impl Intersectable for BVHTree {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.root.bounding_box.intersect(ray).is_none() {
            return None;
        }

        let mut nodes_to_visit = vec![&self.root];

        let mut closest_intersection: Option<Intersection<'_>> = None;

        while let Some(node) = nodes_to_visit.pop() {
            if let Some(bounding_box_intersection) = node.bounding_box.intersect(ray) {
                if node.is_leaf() {
                    if closest_intersection.is_none()
                        || bounding_box_intersection.get_distance()
                            < closest_intersection.as_ref().unwrap().get_distance()
                    {
                        let mesh_intersection = self.mesh.intersect_part(
                            ray,
                            node.start_triangle_index,
                            node.end_triangle_index,
                        );

                        if let Some(mesh_intersection) = mesh_intersection {
                            closest_intersection = Some(mesh_intersection);
                        }
                    }
                    continue;
                }

                if let Some(left) = &node.left {
                    nodes_to_visit.push(left);
                }
                if let Some(right) = &node.right {
                    nodes_to_visit.push(right);
                }
            }
        }

        closest_intersection
    }
}

impl BVHNode {
    pub fn new_from_mesh(
        mesh: &mut Mesh,
        start_triangle_index: usize,
        end_triangle_index: usize,
    ) -> Self {
        let bounding_box = BoundingBox::new_from_mesh_and_triangle_indices(
            mesh,
            start_triangle_index,
            end_triangle_index,
        );

        let diagonals = bounding_box.calculate_diagonals();
        let longest_axis = diagonals.abs().greatest_component();
        let center = bounding_box.calculate_center();

        let mut pivot_index = start_triangle_index;
        for i in start_triangle_index..end_triangle_index {
            let triangle = mesh.get_triangle(i);
            let triangle_center = mesh.calculate_triangle_center(triangle);
            if triangle_center[longest_axis] < center[longest_axis] {
                mesh.swap_triangles(i, pivot_index);
                pivot_index += 1;
            }
        }

        if pivot_index <= start_triangle_index
            || pivot_index >= end_triangle_index - 1
            || end_triangle_index - start_triangle_index <= MIN_BVH_NODE_SIZE
        {
            return BVHNode {
                bounding_box,
                left: None,
                right: None,
                start_triangle_index,
                end_triangle_index,
            };
        }

        let left = BVHNode::new_from_mesh(mesh, start_triangle_index, pivot_index);
        let right = BVHNode::new_from_mesh(mesh, pivot_index, end_triangle_index);

        BVHNode {
            bounding_box,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            start_triangle_index,
            end_triangle_index,
        }
    }

    pub fn get_start_triangle_index(&self) -> usize {
        self.start_triangle_index
    }

    pub fn get_end_triangle_index(&self) -> usize {
        self.end_triangle_index
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
