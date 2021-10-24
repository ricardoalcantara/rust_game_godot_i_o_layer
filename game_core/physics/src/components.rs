use core::components::IdType;

use math::Rectangle2;

pub struct Actor {
    pub remainder_x: f32,
    pub remainder_y: f32,
    pub move_x: f32,
    pub move_y: f32,
    pub riding: Option<i32>,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            remainder_x: 0.0,
            remainder_y: 0.0,
            move_x: 0.0,
            move_y: 0.0,
            riding: None,
        }
    }
}

impl Actor {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Solid {
    pub remainder_x: f32,
    pub remainder_y: f32,
    pub move_x: f32,
    pub move_y: f32,
    pub id: IdType,
}

impl Default for Solid {
    fn default() -> Self {
        Self {
            id: IdType::None,
            remainder_x: 0.0,
            remainder_y: 0.0,
            move_x: 0.0,
            move_y: 0.0,
        }
    }
}

impl Solid {
    pub fn new(id: IdType) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

pub struct Area2D {
    pub rectangle: Rectangle2,
}

impl Area2D {
    pub fn new(rectangle: Rectangle2) -> Self {
        Self { rectangle }
    }
}

pub struct Collision {
    pub rectangle: Rectangle2,
    pub collidable: bool,
}

impl Collision {
    pub fn new(rectangle: Rectangle2) -> Self {
        Self {
            rectangle,
            collidable: true,
        }
    }
}
