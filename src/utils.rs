use glam::Vec2;

pub fn from_u8_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> u32
{
    return u32::from_be_bytes([alpha, red, green, blue]);
}

pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}

pub fn edge_function(p: Vec2, v0: Vec2, v1: Vec2) -> f32
{
    let seg_b = v1 - v0;
    let seg_a = p - v0;

    return seg_a.x * seg_b.y - seg_a.y * seg_b.x;
}

pub fn inside_circle(p: Vec2, center: Vec2, radius: f32) -> bool
{
    let dist_vec = center - p;
    let distance = dist_vec.x * dist_vec.x + dist_vec.y * dist_vec.y;

    return distance <= radius * radius;
}