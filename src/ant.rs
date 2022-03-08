use raylib::prelude::Vector2;
use rand::Rng;
use crate::trail::Trail;

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

    pub fn check_close_trails(&mut self, trail_list: Vec<Trail>, detect_radius: f32) {
        for i in 0..trail_list.len() {
            let trail: &Trail = trail_list.get(i).unwrap();

            let rand_num: f32 = rand::thread_rng().gen_range(0.0..1.0);

            if rand_num > trail.prob_to_follow {
                continue;
            }

            let dist: f32 = 
                f32::powf(self.position.x - trail.position.x, 2.0) + 
                f32::powf(self.position.y - trail.position.y, 2.0);
            
            if dist <= detect_radius * detect_radius {
                let angle: f32 = (self.position.y - trail.position.y)
                                    .atan2(self.position.x - trail.position.x);
                
                // Change x and y directions
                self.velocity.x = angle.cos();
                self.velocity.y = angle.sin();
            }

            // choose one direction per game tick
            break;
        }
    }
}