use glam::{Vec2, Vec3, Vec4, UVec3};
use std::ops::{Add, Mul};

use crate::texture::Texture;

pub struct Mesh
{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<UVec3>,
    pub texture: Option<Texture>,
}

impl Mesh
{
    pub fn new() -> Self
    {
        return Self { vertices: Vec::new(), indices: Vec::new(), texture: None};
    }

    // courtesy of Luca
    pub fn add_section_from_buffers(
        &mut self,
        triangles: &[UVec3],
        positions: &[Vec3],
        normals: &[Vec3],
        colors: &[Vec3],
        uvs: &[Vec2],
    ) 
    {
        self.indices.extend_from_slice(triangles);

        let has_uvs = !uvs.is_empty();
        let has_colors = !colors.is_empty();

        for i in 0..positions.len() 
        {
            let vertex = Vertex::new(
                positions[i].extend(1.0),
                normals[i],
                if has_colors { colors[i] } else { Vec3::ONE },
                if has_uvs { uvs[i] } else { Vec2::ZERO },
            );
            self.vertices.push(vertex);
        }
    }

    // courtesy of Luca
    pub fn load_from_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Mesh 
    {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut indices = vec![];
        // TODO: handle errors
        let mut result = Mesh::new();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| indices.push(i));
            }
            if let Some(positions_reader) = reader.read_positions() {
                positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
            }
            if let Some(normals_reader) = reader.read_normals() {
                normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
            }
            if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                tex_coord_reader
                    .into_f32()
                    .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
            }

            let colors: Vec<Vec3> = positions.iter().map(|_| Vec3::ONE).collect();
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());

            let triangles: Vec<UVec3> = indices
            .chunks_exact(3)
            .map(|tri| UVec3::new(tri[0], tri[1], tri[2]))
            .collect();
            result.add_section_from_buffers(&triangles, &positions, &normals, &colors, &tex_coords);
        }
        return result;
    }
}

#[derive(Clone, Copy)]
pub struct Vertex
{
    pub pos: Vec4,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Vertex
{
    pub fn new(position: Vec4, normal: Vec3, color: Vec3, uv: Vec2) -> Self 
    {
        return Self { pos: position, normal: normal, color: color, uv: uv };
    }
}

impl Mul<f32> for Vertex
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self
    {
        let position = self.pos * rhs;
        let normal = self.normal * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;

        return Self { pos: position, normal: normal, color: color, uv: uv };
    }
}

impl Add for Vertex
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        let position = self.pos + rhs.pos;
        let normal = self.normal + rhs.normal;
        let color = self.color + self.color;
        let uv = self.uv + rhs.uv;

        return Self { pos: position, normal: normal, color: color, uv: uv };
    }
}