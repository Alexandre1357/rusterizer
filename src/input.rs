use glam::{Vec2, Vec3, Vec3Swizzles};
use minifb::Window;

use crate::utils::*;

pub struct MouseState
{
    pub pos: Vec2,
    pub left_button: bool,
    pub has_selected: bool,
}

impl MouseState
{
    pub fn current(window: &Window, upscale: usize, item_selected: bool) -> MouseState
    {
        let (x, y) = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();

        return MouseState { pos: Vec2::new(x / upscale as f32, y / upscale as f32), left_button: window.get_mouse_down(minifb::MouseButton::Left), has_selected: item_selected };
    }
}

#[derive(Clone, PartialEq, Eq)]
enum PointState
{
    None,
    Hovered,
    Held,
}

#[derive(Clone)]
pub struct MoveablePoint
{
    radius: f32,
    state: PointState,
}

impl MoveablePoint
{
    pub fn new() -> MoveablePoint
    {
        return MoveablePoint { radius: 1.5, state: PointState::None };
    }

    pub fn update(&mut self, pos: &mut Vec3, mouse: &mut MouseState)
    {
        match self.state
        {
            PointState::None =>
            {
                if inside_circle(mouse.pos, pos.xy(), self.radius)
                {
                    self.radius = 2.5;
                    self.state = PointState::Hovered;
                }
            }

            PointState::Hovered =>
            {
                if !inside_circle(mouse.pos, pos.xy(), self.radius)
                {
                    self.radius = 1.5;
                    self.state = PointState::None;
                }

                if mouse.left_button && !mouse.has_selected
                {
                    self.state = PointState::Held;
                    mouse.has_selected = true;
                }
            }

            PointState::Held =>
            {
                if !mouse.left_button
                {
                    self.state = PointState::Hovered;
                    mouse.has_selected = false;
                }

                pos.x = mouse.pos.x;
                pos.y = mouse.pos.y;
            }
        }
    }
}