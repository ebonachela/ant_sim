use raylib::prelude::{ Vector2, Color };

#[derive(PartialEq, Clone, Copy)]
pub struct Trail {
    pub position: Vector2,
    pub color: Color,
    pub counter: i32,
    pub prob_to_follow: f32
}

impl Trail {
    pub fn get_prob_to_follow(
        &self, trail_list: &mut Vec<Trail>,
        trail_check_radius: f32, 
        trail_inc_prob_value: f32
    ) {
        for i in 0..trail_list.len() {
            let trail: &mut Trail = trail_list.get_mut(i).unwrap();

            if trail == self { continue; }

            let dist: f32 = 
                f32::powf(self.position.x - trail.position.x, 2.0) + 
                f32::powf(self.position.y - trail.position.y, 2.0);

            if dist <= trail_check_radius * trail_check_radius {
                trail.prob_to_follow += trail_inc_prob_value;
            }
        }
    }
}