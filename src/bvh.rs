use crate::caster::{BIAS, Intersection, Ray, RayIntersect};
use crate::cube::Cube;
use crate::material::Light;
use crate::vector::Vec3A;

pub struct Bvh {
    objects: Vec<Cube>,
    indices: Vec<usize>,
    nodes: Vec<Node>,
}

struct Node {
    min: Vec3A,
    max: Vec3A,
    kind: NodeKind,
}

enum NodeKind {
    Leaf { start: usize, end: usize },
    Branch { left: usize, right: usize },
}

impl Node {
    fn entry(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        let mut near = BIAS;
        let mut far = max_distance;
        for axis in 0..3 {
            // Avoid 0 * infinity (NaN) for rays on a face parallel to this axis.
            if ray.direction[axis] == 0.0 {
                if ray.origin[axis] < self.min[axis] || ray.origin[axis] > self.max[axis] {
                    return None;
                }
            } else {
                let a = (self.min[axis] - ray.origin[axis]) * ray.inv_direction[axis];
                let b = (self.max[axis] - ray.origin[axis]) * ray.inv_direction[axis];
                near = near.max(a.min(b));
                far = far.min(a.max(b));
            }
            if near > far {
                return None;
            }
        }
        Some(near)
    }
}

impl Bvh {
    pub fn new(objects: Vec<Cube>) -> Self {
        let count = objects.len();
        let mut bvh = Self {
            objects,
            indices: (0..count).collect(),
            nodes: Vec::new(),
        };
        if count != 0 {
            bvh.build(0, count);
        }
        bvh
    }

    fn build(&mut self, start: usize, end: usize) -> usize {
        let mut min = Vec3A::splat(f32::INFINITY);
        let mut max = Vec3A::splat(f32::NEG_INFINITY);
        for &index in &self.indices[start..end] {
            min = min.min(self.objects[index].min_b);
            max = max.max(self.objects[index].max_b);
        }
        let node = self.nodes.len();
        self.nodes.push(Node {
            min,
            max,
            kind: NodeKind::Leaf { start, end },
        });
        if end - start > 4 {
            let extent = max - min;
            let axis = if extent.x >= extent.y && extent.x >= extent.z {
                0
            } else if extent.y >= extent.z {
                1
            } else {
                2
            };
            let middle = start + (end - start) / 2;
            let objects = &self.objects;
            self.indices[start..end].select_nth_unstable_by(middle - start, |&a, &b| {
                objects[a].center[axis]
                    .total_cmp(&objects[b].center[axis])
                    .then(a.cmp(&b))
            });
            let left = self.build(start, middle);
            let right = self.build(middle, end);
            self.nodes[node].kind = NodeKind::Branch { left, right };
        }
        node
    }

    pub fn closest_hit(&self, ray: &Ray) -> Intersection {
        let mut hit = Intersection::no_intersection();
        let mut best_index = usize::MAX;
        if let Some(entry) = self.nodes.first().and_then(|n| n.entry(ray, hit.distance)) {
            self.visit(0, entry, ray, &mut hit, &mut best_index);
        }
        hit
    }

    pub fn emissive_lights(&self) -> impl Iterator<Item = Light> + '_ {
        self.objects
            .iter()
            .filter(|cube| cube.material.emission_strength > 0.0)
            .map(|cube| Light {
                position: cube.center,
                intensity: cube.material.emission_strength,
                color: cube.material.emission_color,
                attenuation: 0.1,
            })
    }

    fn visit(
        &self,
        index: usize,
        entry: f32,
        ray: &Ray,
        hit: &mut Intersection,
        best_index: &mut usize,
    ) {
        if entry > hit.distance {
            return;
        }
        match self.nodes[index].kind {
            NodeKind::Leaf { start, end } => {
                for &object_index in &self.indices[start..end] {
                    let candidate = self.objects[object_index].intersect(ray);
                    // Preserve original scene order for equally distant surfaces.
                    if candidate.is_intersecting
                        && candidate.distance > BIAS
                        && (candidate.distance < hit.distance
                            || (hit.is_intersecting
                                && candidate.distance == hit.distance
                                && object_index < *best_index))
                    {
                        *hit = candidate;
                        *best_index = object_index;
                    }
                }
            }
            NodeKind::Branch { left, right } => {
                let a = self.nodes[left].entry(ray, hit.distance);
                let b = self.nodes[right].entry(ray, hit.distance);
                match (a, b) {
                    (Some(a), Some(b)) => {
                        let children = if a <= b {
                            [(left, a), (right, b)]
                        } else {
                            [(right, b), (left, a)]
                        };
                        for (child, entry) in children {
                            self.visit(child, entry, ray, hit, best_index);
                        }
                    }
                    (Some(a), None) => self.visit(left, a, ray, hit, best_index),
                    (None, Some(b)) => self.visit(right, b, ray, hit, best_index),
                    (None, None) => {}
                }
            }
        }
    }

    pub fn is_occluded(&self, ray: &Ray, max_distance: f32) -> bool {
        let light_position = ray.origin + ray.direction * max_distance;
        !self.nodes.is_empty() && self.occluded_node(0, ray, max_distance, light_position)
    }

    fn occluded_node(
        &self,
        index: usize,
        ray: &Ray,
        max_distance: f32,
        light_position: Vec3A,
    ) -> bool {
        let node = &self.nodes[index];
        if node.entry(ray, max_distance).is_none() {
            return false;
        }
        match node.kind {
            NodeKind::Leaf { start, end } => self.indices[start..end].iter().any(|&index| {
                let object = &self.objects[index];
                let hit = object.intersect(ray);
                let contains_light = light_position.x >= object.min_b.x
                    && light_position.x <= object.max_b.x
                    && light_position.y >= object.min_b.y
                    && light_position.y <= object.max_b.y
                    && light_position.z >= object.min_b.z
                    && light_position.z <= object.max_b.z;
                hit.is_intersecting
                    && hit.distance > BIAS
                    && hit.distance < max_distance
                    && !contains_light
            }),
            NodeKind::Branch { left, right } => {
                self.occluded_node(left, ray, max_distance, light_position)
                    || self.occluded_node(right, ray, max_distance, light_position)
            }
        }
    }
}
