pub type Vector2 = euclid::default::Vector2D<f32>;
// pub type Transform2D = euclid::default::Transform2D<f32>;
// pub type Quat = euclid::default::Rotation3D<f32>;
pub type Size2 = euclid::default::Size2D<f32>;
pub type Rectangle2 = euclid::default::Rect<f32>;
pub type Angle = euclid::Angle<f32>;
// pub type Point3 = euclid::default::Point3D<f32>;
pub type Point2 = euclid::default::Point2D<f32>;
pub type Rotation2D = euclid::default::Rotation2D<f32>;
// pub type Rotation3D = euclid::default::Rotation3D<f32>;

pub struct Transform2 {
    pub position: Vector2,
}

impl Transform2 {
    pub fn new(position: Vector2) -> Self {
        Self { position }
    }
}

impl Default for Transform2 {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
        }
    }
}
