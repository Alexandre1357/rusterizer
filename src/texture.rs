pub use std::path::Path;

use crate::utils::*;

pub struct Texture
{
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

impl Texture
{
    pub fn load(path: &Path) -> Self
    {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image
        {
            let data = (0..image.data.len() / 3)
            .map(|id| 
            {
                from_u8_rgba(
                    image.data[id * 3],
                    image.data[id * 3 + 1],
                    image.data[id * 3 + 2],
                    255
                )
            })
            .collect();

            return Self
            {
                width: image.width,
                height: image.height,
                data,
            };
        }
        else 
        {
            panic!("File not loaded");
        }
    }
}