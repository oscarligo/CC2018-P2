use crate::caster::{Intersection, Ray, RayIntersect};
use crate::material::Material;
use crate::vector::Vec3A;
use std::fs;

#[derive(Clone, Copy)]
pub struct Triangle {
    vertices: [Vec3A; 3],
    normals: [Vec3A; 3],
    uvs: [(f32, f32); 3],
    pub material: Material,
    pub min_b: Vec3A,
    pub max_b: Vec3A,
    pub center: Vec3A,
    tangent: Vec3A,
    bitangent: Vec3A,
    animation_pivot: Vec3A,
    animation_phase: f32,
}

impl Triangle {
    fn new(
        vertices: [Vec3A; 3],
        normals: [Vec3A; 3],
        uvs: [(f32, f32); 3],
        material: Material,
        animation_pivot: Vec3A,
        animation_phase: f32,
    ) -> Self {
        let edge = vertices[1] - vertices[0];
        let face_normal = edge.cross(vertices[2] - vertices[0]).normalize();
        let tangent = edge.normalize();
        let bitangent = face_normal.cross(tangent).normalize();
        let min_b = vertices[0].min(vertices[1]).min(vertices[2]);
        let max_b = vertices[0].max(vertices[1]).max(vertices[2]);

        Self {
            vertices,
            normals,
            uvs,
            material,
            min_b,
            max_b,
            center: (vertices[0] + vertices[1] + vertices[2]) / 3.0,
            tangent,
            bitangent,
            animation_pivot,
            animation_phase,
        }
    }

    pub fn translated(&self, offset: Vec3A) -> Self {
        let mut triangle = *self;
        triangle.vertices = triangle.vertices.map(|vertex| vertex + offset);
        triangle.min_b += offset;
        triangle.max_b += offset;
        triangle.center += offset;
        triangle
    }

    pub fn rotated_y(&self, angle: f32) -> Self {
        let mut triangle = *self;
        triangle.vertices = triangle
            .vertices
            .map(|vertex| vertex.rotate_around(Vec3A::Y, angle));
        triangle.normals = triangle
            .normals
            .map(|normal| normal.rotate_around(Vec3A::Y, angle));
        triangle.tangent = triangle.tangent.rotate_around(Vec3A::Y, angle);
        triangle.bitangent = triangle.bitangent.rotate_around(Vec3A::Y, angle);
        triangle.min_b = triangle.vertices[0]
            .min(triangle.vertices[1])
            .min(triangle.vertices[2]);
        triangle.max_b = triangle.vertices[0]
            .max(triangle.vertices[1])
            .max(triangle.vertices[2]);
        triangle.center =
            (triangle.vertices[0] + triangle.vertices[1] + triangle.vertices[2]) / 3.0;
        triangle
    }

    pub fn animated(&self, angle: f32) -> Self {
        if self.animation_phase == 0.0 || angle == 0.0 {
            return *self;
        }

        let mut triangle = *self;
        let angle = angle * self.animation_phase;
        let axis = Vec3A::new(0.0, 0.0, 1.0);
        triangle.vertices = triangle.vertices.map(|vertex| {
            (vertex - self.animation_pivot).rotate_around(axis, angle) + self.animation_pivot
        });
        triangle.normals = triangle
            .normals
            .map(|normal| normal.rotate_around(axis, angle));
        triangle.tangent = triangle.tangent.rotate_around(axis, angle);
        triangle.bitangent = triangle.bitangent.rotate_around(axis, angle);
        triangle.min_b = triangle.vertices[0]
            .min(triangle.vertices[1])
            .min(triangle.vertices[2]);
        triangle.max_b = triangle.vertices[0]
            .max(triangle.vertices[1])
            .max(triangle.vertices[2]);
        triangle.center =
            (triangle.vertices[0] + triangle.vertices[1] + triangle.vertices[2]) / 3.0;
        triangle
    }
}

impl RayIntersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let edge_1 = self.vertices[1] - self.vertices[0];
        let edge_2 = self.vertices[2] - self.vertices[0];
        let p = ray.direction.cross(edge_2);
        let determinant = edge_1.dot(p);

        if determinant.abs() < 1e-8 {
            return Intersection::no_intersection();
        }

        let inverse_determinant = 1.0 / determinant;
        let from_vertex = ray.origin - self.vertices[0];
        let u = from_vertex.dot(p) * inverse_determinant;
        if !(0.0..=1.0).contains(&u) {
            return Intersection::no_intersection();
        }

        let q = from_vertex.cross(edge_1);
        let v = ray.direction.dot(q) * inverse_determinant;
        if v < 0.0 || u + v > 1.0 {
            return Intersection::no_intersection();
        }

        let distance = edge_2.dot(q) * inverse_determinant;
        if distance <= 0.001 {
            return Intersection::no_intersection();
        }

        let w = 1.0 - u - v;
        let mut normal =
            (self.normals[0] * w + self.normals[1] * u + self.normals[2] * v).normalize();
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }
        let texture_u = self.uvs[0].0 * w + self.uvs[1].0 * u + self.uvs[2].0 * v;
        let texture_v = self.uvs[0].1 * w + self.uvs[1].1 * u + self.uvs[2].1 * v;
        let point = ray.origin + ray.direction * distance;

        Intersection::new(
            distance,
            point,
            normal,
            true,
            self.material,
            (texture_u, texture_v),
            self.tangent,
            self.bitangent,
        )
    }
}

pub fn load_obj(path: &str, height: f32, material: Material) -> Result<Vec<Triangle>, String> {
    let source = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    parse_obj(&source, height, material).map_err(|error| format!("{path}: {error}"))
}

fn parse_obj(source: &str, height: f32, material: Material) -> Result<Vec<Triangle>, String> {
    if height <= 0.0 {
        return Err("la altura del modelo debe ser positiva".to_string());
    }

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut faces = Vec::new();
    let mut group = 0;

    for (line_index, line) in source.lines().enumerate() {
        let line_number = line_index + 1;
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.first().copied() {
            Some("g") => group += 1,
            Some("v") => vertices.push(parse_vec3(&parts, line_number)?),
            Some("vn") => normals.push(parse_vec3(&parts, line_number)?.normalize()),
            Some("vt") => {
                if parts.len() < 3 {
                    return Err(format!("línea {line_number}: coordenada UV incompleta"));
                }
                let u = parse_number(parts[1], line_number)?;
                let v = parse_number(parts[2], line_number)?;
                uvs.push((u, 1.0 - v));
            }
            Some("f") => {
                if parts.len() != 4 {
                    return Err(format!("línea {line_number}: la cara no es un triángulo"));
                }
                faces.push((
                    [
                        parse_face_vertex(parts[1], line_number)?,
                        parse_face_vertex(parts[2], line_number)?,
                        parse_face_vertex(parts[3], line_number)?,
                    ],
                    group,
                ));
            }
            _ => {}
        }
    }

    if vertices.is_empty() || faces.is_empty() {
        return Err("el modelo no contiene vértices y caras".to_string());
    }

    let min = vertices
        .iter()
        .copied()
        .fold(Vec3A::splat(f32::INFINITY), Vec3A::min);
    let max = vertices
        .iter()
        .copied()
        .fold(Vec3A::splat(f32::NEG_INFINITY), Vec3A::max);
    let model_height = max.y - min.y;
    if model_height <= f32::EPSILON {
        return Err("el modelo no tiene altura".to_string());
    }
    let scale = height / model_height;
    let origin = Vec3A::new((min.x + max.x) * 0.5, min.y, (min.z + max.z) * 0.5);
    let mut group_bounds = vec![
        (
            Vec3A::splat(f32::INFINITY),
            Vec3A::splat(f32::NEG_INFINITY),
        );
        group + 1
    ];
    for (face, group) in &faces {
        for [vertex, _, _] in face {
            let position = (vertices[*vertex] - origin) * scale;
            group_bounds[*group].0 = group_bounds[*group].0.min(position);
            group_bounds[*group].1 = group_bounds[*group].1.max(position);
        }
    }

    faces
        .into_iter()
        .map(|(face, group)| {
            let mut positions = [Vec3A::ZERO; 3];
            let mut face_normals = [Vec3A::ZERO; 3];
            let mut face_uvs = [(0.0, 0.0); 3];
            for (corner, [vertex, uv, normal]) in face.into_iter().enumerate() {
                positions[corner] = (*vertices
                    .get(vertex)
                    .ok_or_else(|| format!("índice de vértice {} fuera de rango", vertex + 1))?
                    - origin)
                    * scale;
                face_uvs[corner] = *uvs
                    .get(uv)
                    .ok_or_else(|| format!("índice UV {} fuera de rango", uv + 1))?;
                face_normals[corner] = *normals
                    .get(normal)
                    .ok_or_else(|| format!("índice normal {} fuera de rango", normal + 1))?;
            }
            // shortcut: character.obj orders limbs as groups 3-6; use named groups if the model changes.
            let animation_phase = match group {
                3 | 4 => 1.0,
                5 | 6 => -1.0,
                _ => 0.0,
            };
            let (part_min, part_max) = group_bounds[group];
            let animation_pivot = Vec3A::new(
                (part_min.x + part_max.x) * 0.5,
                part_max.y,
                (part_min.z + part_max.z) * 0.5,
            );
            Ok(Triangle::new(
                positions,
                face_normals,
                face_uvs,
                material,
                animation_pivot,
                animation_phase,
            ))
        })
        .collect()
}

fn parse_vec3(parts: &[&str], line: usize) -> Result<Vec3A, String> {
    if parts.len() < 4 {
        return Err(format!("línea {line}: vector incompleto"));
    }
    Ok(Vec3A::new(
        parse_number(parts[1], line)?,
        parse_number(parts[2], line)?,
        parse_number(parts[3], line)?,
    ))
}

fn parse_number(value: &str, line: usize) -> Result<f32, String> {
    value
        .parse()
        .map_err(|_| format!("línea {line}: número inválido '{value}'"))
}

fn parse_face_vertex(value: &str, line: usize) -> Result<[usize; 3], String> {
    let indices: Vec<_> = value.split('/').collect();
    if indices.len() != 3 {
        return Err(format!("línea {line}: se esperaba vértice/uv/normal"));
    }
    let mut parsed = [0; 3];
    for (target, index) in parsed.iter_mut().zip(indices) {
        *target = index
            .parse::<usize>()
            .ok()
            .and_then(|index| index.checked_sub(1))
            .ok_or_else(|| format!("línea {line}: índice OBJ inválido '{index}'"))?;
    }
    Ok(parsed)
}
