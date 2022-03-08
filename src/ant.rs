use raylib::prelude::Vector2;

#[derive(Copy, Clone)]
pub struct Ant {
    pub position: Vector2,
    pub velocity: Vector2,
    pub speed: f32,
    pub radius: f32
}

impl Ant {
    pub fn check_wall_collision(&mut self, width: i32, height: i32) {
        if (width as f32) < (self.position.x + self.radius) || 
           (self.position.x - self.radius) < 0.0 {
            self.velocity.x *= -1.0;
        }

        if (height as f32) < (self.position.y + self.radius) || 
           (self.position.y - self.radius) < 0.0 {
            self.velocity.y *= -1.0;
        }
    }
}