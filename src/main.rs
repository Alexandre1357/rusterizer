use glam::{Vec2, Vec3, UVec3, Vec4, Mat4, Quat};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;

mod transform;
use transform::Transform;

use rusterizer::*;

const WIDTH:  usize = 1920;
const HEIGHT: usize = 1080;

const UPSCALE: usize = 1;

fn main() 
{
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut z_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    let mut upscale_buffer: Vec<u32> = vec![0; WIDTH*UPSCALE * HEIGHT*UPSCALE];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH*UPSCALE,
        HEIGHT*UPSCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| 
    {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let test_albedo = Texture::load(Path::new("assets/test.jpg"));
    let helmet_albedo = Texture::load(Path::new("assets/helmet_albedo.jpg"));

    let mut meshes = 
    [
        Mesh
        { 
            vertices: 
            vec![
                Vertex{ pos: Vec4::new(-1.0, -1.0, 0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 0.0), color: Vec3::new(1.0, 1.0, 1.0) },
                Vertex{ pos: Vec4::new( 1.0, -1.0, 0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(1.0, 0.0), color: Vec3::new(1.0, 1.0, 1.0) },
                Vertex{ pos: Vec4::new(-1.0,  1.0, 0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 1.0), color: Vec3::new(0.0, 0.0, 0.0) },
                Vertex{ pos: Vec4::new( 1.0,  1.0, 0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(1.0, 1.0), color: Vec3::new(1.0, 1.0, 1.0) }
            ],

            indices:
            vec![
                UVec3 { x: 0, y: 1, z: 2},
                UVec3 { x: 1, y: 2, z: 3}
            ],

            texture: Some(test_albedo)
        },
        load_gltf(Path::new("assets/DamagedHelmet.gltf"))
    ];

    meshes[1].texture = Some(helmet_albedo);

    let mut transforms = vec![Transform::IDENTITY; meshes.len()];

    transforms[0].translation.z -= 11.0;
    transforms[0].translation.x -= 8.0;
    transforms[0].scale *= 3.0;
    let mut rot0 = 0.0;
    
    transforms[1].translation.z -= 15.0;
    //transforms[1].translation.x -= 3.0;
    transforms[1].scale *= 2.0;
    let mut rot1 = 0.0;

    let mut eye = Vec3::new(0.0, 0.0, 0.0);

    let perspective = Mat4::perspective_rh(std::f32::consts::PI / 4.0, WIDTH as f32 / HEIGHT as f32, 5.0, 100.0);

    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        buffer.fill(0);
        z_buffer.fill(f32::INFINITY);

        update_camera_eye(&mut eye, &window);

        transforms[0].rotation = Quat::from_euler(glam::EulerRot::XYZ, rot0, 0.0, 0.0);
        rot0 += 0.005;

        transforms[1].rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, rot1, 0.0);
        rot1 -= 0.005;

        let view = Mat4::look_at_rh(eye, eye + Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0));

        // render meshes
        for (m_i, mesh) in meshes.iter().enumerate()
        {
            let model = Mat4::from_translation(transforms[m_i].translation) * Mat4::from_quat(transforms[m_i].rotation) * Mat4::from_scale(transforms[m_i].scale);
            let mvp = perspective * view * model;

            for (_t_i, vertex_indices) in mesh.indices.iter().enumerate()
            {
                clip_and_rasterize_triangle([mesh.vertices[vertex_indices.x as usize], mesh.vertices[vertex_indices.y as usize], mesh.vertices[vertex_indices.z as usize]], mvp, 
                                    &mesh.texture, &mut buffer, &mut z_buffer, (WIDTH, HEIGHT));
            }
        }

        // upscale resolution
        for i in 0..(WIDTH*UPSCALE*HEIGHT*UPSCALE)
        {
            let x_up = i % (WIDTH*UPSCALE);
            let y_up = i / (WIDTH*UPSCALE);

            let x = x_up / UPSCALE;
            let y = y_up / UPSCALE;

            let buffer_index = x + y * WIDTH;

            upscale_buffer[i] = buffer[buffer_index];
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn update_camera_eye(eye: &mut Vec3, window: &Window)
{
    if window.is_key_down(Key::Left)
    {
        eye.x -= 0.5;
    }

    if window.is_key_down(Key::Right)
    {
        eye.x += 0.5;
    }

    if window.is_key_down(Key::Space)
    {
        eye.y += 0.5;
    }

    if window.is_key_down(Key::LeftShift)
    {
        eye.y -= 0.5;
    }

    if window.is_key_down(Key::Up)
    {
        eye.z -= 0.5;
    }

    if window.is_key_down(Key::Down)
    {
        eye.z += 0.5;
    }
}