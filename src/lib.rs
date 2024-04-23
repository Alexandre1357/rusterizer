use glam::{Mat4, Vec3, Vec2, UVec2};
use minifb::clamp;
use std::path::Path;
use std::cmp::{min, max};

pub mod input;
pub mod geometry;
pub mod utils;
pub mod texture;
pub use 
{
    input::MouseState,
    geometry::Mesh,
    geometry::Vertex,
    texture::Texture,
    utils::*,
};

pub fn clip_and_rasterize_triangle(triangle_original: [Vertex; 3], mvp: Mat4, texture: &Option<Texture>, buffer: &mut Vec<u32>, z_buffer: &mut Vec<f32>, viewport: (usize, usize))
{
    let mut triangle_projected = triangle_original;

    triangle_projected[0].pos = mvp * triangle_original[0].pos;
    triangle_projected[1].pos = mvp * triangle_original[1].pos;
    triangle_projected[2].pos = mvp * triangle_original[2].pos;

    let mut new_order = [0; 3];
    let mut num_valid = 0;
    let mut num_invalid = 0;

    for (i, vertex) in triangle_projected.iter().enumerate()
    {
        if vertex.pos.z < 0.0
        {
            let new_index = 2 - num_invalid;
            new_order[new_index] = i;
            num_invalid += 1;
        }
        else
        {
            let new_index = num_valid;
            new_order[new_index] = i;
            num_valid += 1;
        }
    }

    let triangle_ordered = [triangle_projected[new_order[0]], triangle_projected[new_order[1]], triangle_projected[new_order[2]]];

    match num_invalid 
    {
        3 => return,

        2 => 
        {
            let alpha01 = -triangle_ordered[0].pos.z / (triangle_ordered[1].pos.z - triangle_ordered[0].pos.z);
            let alpha02 = -triangle_ordered[0].pos.z / (triangle_ordered[2].pos.z - triangle_ordered[0].pos.z);

            let prime1 = triangle_ordered[0] * (1.0 - alpha01) + triangle_ordered[1] * alpha01;
            let prime2 = triangle_ordered[0] * (1.0 - alpha02) + triangle_ordered[2] * alpha02;

            let mut triangle_local = triangle_ordered;

            triangle_local[1] = prime1;
            triangle_local[2] = prime2;

            // visual testing
            let red = Vec3::new(1.0, 0.0, 0.0);

            triangle_local[0].color = red;
            triangle_local[1].color = red;
            triangle_local[2].color = red;

            rasterize_triangle(triangle_local,  texture, buffer, z_buffer, viewport)
        }

        1 =>
        {
            let alpha02 = -triangle_ordered[0].pos.z / (triangle_ordered[2].pos.z - triangle_ordered[0].pos.z);
            let alpha12 = -triangle_ordered[1].pos.z / (triangle_ordered[2].pos.z - triangle_ordered[1].pos.z);

            let prime0 = triangle_ordered[0] * (1.0 - alpha02) + triangle_ordered[2] * alpha02;
            let prime1 = triangle_ordered[1] * (1.0 - alpha12) + triangle_ordered[2] * alpha12;

            let mut tri0 = [triangle_ordered[0], triangle_ordered[1], prime0];
            let mut tri1 = [prime0, triangle_ordered[1], prime1];

            // visual testing
            let green = Vec3::new(0.0, 1.0, 0.0);
            let blue = Vec3::new(0.0, 0.0, 1.0);

            tri0[0].color = green;
            tri0[1].color = green;
            tri0[2].color = green;

            tri1[0].color = blue;
            tri1[1].color = blue;
            tri1[2].color = blue;
            
            rasterize_triangle(tri0, texture, buffer, z_buffer, viewport);
            rasterize_triangle(tri1, texture, buffer, z_buffer, viewport);
        }

        0 =>
        {
            rasterize_triangle(triangle_ordered, texture, buffer, z_buffer, viewport);
        }

        _ =>
        {

        }
    }

    
}

fn rasterize_triangle(triangle: [Vertex; 3], texture: &Option<Texture>, buffer: &mut Vec<u32>, z_buffer: &mut Vec<f32>, viewport: (usize, usize))
{
    let rec0 = 1.0 / triangle[0].pos.w;
    let rec1 = 1.0 / triangle[1].pos.w;
    let rec2 = 1.0 / triangle[2].pos.w;

    let ndc0 = triangle[0].pos * rec0;
    let ndc1 = triangle[1].pos * rec1;
    let ndc2 = triangle[2].pos * rec2;

    let v0 = triangle[0] * rec0;
    let v1 = triangle[1] * rec1;
    let v2 = triangle[2] * rec2;

    let sc0 = Vec2::new(map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport.0 as f32), map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport.1 as f32));
    let sc1 = Vec2::new(map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport.0 as f32), map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport.1 as f32));
    let sc2 = Vec2::new(map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport.0 as f32), map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport.1 as f32));

    let area = edge_function(sc0, sc1, sc2);

    let clamped_sc0 = UVec2::new(clamp(0, sc0.x as i32, viewport.0 as i32 - 1) as u32, clamp(0, sc0.y as i32, viewport.1 as i32 - 1) as u32);
    let clamped_sc1 = UVec2::new(clamp(0, sc1.x as i32, viewport.0 as i32 - 1) as u32, clamp(0, sc1.y as i32, viewport.1 as i32 - 1) as u32);
    let clamped_sc2 = UVec2::new(clamp(0, sc2.x as i32, viewport.0 as i32 - 1) as u32, clamp(0, sc2.y as i32, viewport.1 as i32 - 1) as u32);

    let lower_bounds = UVec2::new(min(min(clamped_sc0.x, clamped_sc1.x), clamped_sc2.x), 
                                         min(min(clamped_sc0.y, clamped_sc1.y), clamped_sc2.y));
    let upper_bounds = UVec2::new(max(max(clamped_sc0.x, clamped_sc1.x), clamped_sc2.x), 
                                         max(max(clamped_sc0.y, clamped_sc1.y), clamped_sc2.y));

    let width = upper_bounds.x as usize - lower_bounds.x as usize + 1;                 
    let height = upper_bounds.y as usize - lower_bounds.y as usize + 1;

    for frame_pixel_i in 0..(width * height)
    {
        let local_coords = (frame_pixel_i % width, frame_pixel_i / width);
        let coords = (local_coords.0 + lower_bounds.x as usize, local_coords.1 + lower_bounds.y as usize);
        let p_i = coords.1 * viewport.0 + coords.0;
        let point = Vec2::new(coords.0 as f32, coords.1 as f32);

        let a = edge_function(point, sc1, sc2) / area;
        let b = edge_function(point, sc2, sc0) / area;
        let c = edge_function(point, sc0, sc1) / area;

        if a >= 0.0 && b >= 0.0 && c >= 0.0
        {
            let correction = a * rec0 + b * rec1 + c * rec2;
                        
            let correction = 1.0 / correction;
            let depth = correction;

            if depth < z_buffer[p_i]
            {
                z_buffer[p_i] = depth;

                let color;
                       
                if let Some(texture) = texture
                {
                    let mut uv = v0.uv * a;
                    uv += v1.uv * b;
                    uv += v2.uv * c;
                    uv *= correction;

                    //uv.x = clamp(0.0, uv.x, 1.0);
                    //uv.y = clamp(0.0, uv.y, 1.0);

                    let texture_x = (uv.x * texture.width as f32) as usize % texture.width;
                    let texture_y = (uv.y * texture.height as f32) as usize % texture.height;
                    
                    let texture_index = texture_x + texture_y * texture.width;
                    color = texture.data[clamp(0, texture_index, texture.data.len() - 1)];
                }
                else 
                {
                    let mut rgb = v0.color * a;
                    rgb += v1.color * b;
                    rgb += v2.color * c;
                    rgb *= correction;

                    color = from_u8_rgba((rgb.x * 255.0) as u8, (rgb.y * 255.0) as u8, (rgb.z * 255.0) as u8, 255);
                }

                buffer[p_i] = color;
            }
        }
    }
}

// courtesy of Luca
pub fn load_gltf(path: &Path) -> Mesh 
{
    // handle loading textures, cameras, meshes here
    let (document, buffers, _images) = gltf::import(path).unwrap();

    for scene in document.scenes() 
    {
        for node in scene.nodes() 
        {
            println!(
                "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                node.index(),
                node.children().count(),
                node.camera(),
                node.mesh().is_some(),
                node.transform(),
            );
            println!(
                "Node #{} has transform: trans {:?}, rot {:?}, scale {:?},",
                node.index(),
                node.transform().decomposed().0,
                node.transform().decomposed().1,
                node.transform().decomposed().2,
            );
            if let Some(mesh) = node.mesh() 
            {
                return Mesh::load_from_gltf(&mesh, &buffers);
            }
        }
    }

    Mesh::new()
}