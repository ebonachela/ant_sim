use raylib::prelude::{Vector2, Color};

#[derive(Clone)]
pub enum TargetType {
    Food {
        position: Vector2,
        radius: f32,
        count: i32,
        color: Color
    },
    Base {
        position: Vector2,
        radius: f32,
        color: Color
    }
}