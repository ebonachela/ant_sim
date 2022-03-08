use raylib::prelude::*;
use rand::Rng;

mod ant;
mod trail;

// Global constants
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 960;
const TARGET_FPS: u32 = 60;
const FPS_TEXT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;

// Ants constants
const ANT_SPEED: f32 = 100.0;
const ANT_RADIUS: f32 = 1.0;
const ANT_COUNT: i32 = 10;
const ANT_COLOR: Color = Color::WHITE;

// Trail constants
const TRAIL_BASE_COLOR: Color = Color::YELLOW;
const TRAIL_ALPHA_MULTIPLIER: i32 = 10000;
const TRAIL_CONSUME_SPEED: i32 = 10000;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Ant Simulation")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut trail_list: Vec<trail::Trail> = Vec::new();
    let mut ant_list: Vec<ant::Ant> = Vec::new();
    let mut rng = rand::thread_rng();

    // Generate ant list
    for _ in 0..ANT_COUNT {
        let mut velocity_x: f32 = rng.gen_range(-1.0..1.0);
        let mut velocity_y: f32 = rng.gen_range(-1.0..1.0);

        if velocity_x == 0.0 { velocity_x = 1.0; };
        if velocity_y == 0.0 { velocity_y = 1.0; };

        ant_list.push(ant::Ant{
            position: Vector2 {
                x: (WIDTH / 2) as f32,
                y: (HEIGHT / 2) as f32
            },
            velocity: Vector2 {
                x: velocity_x, 
                y: velocity_y 
            },
            speed: ANT_SPEED,
            radius: ANT_RADIUS
        });
    }
   
    // Draw to screen
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let mut fps_text: String = String::from("FPS: ");
        let fps_rate: String = d.get_fps().to_string();
        fps_text.push_str(&fps_rate);
        
        // Draw background and fps text
        d.clear_background(BACKGROUND_COLOR);
        d.draw_text("Ant Simulation", 12, 12, 20, FPS_TEXT_COLOR);
        d.draw_text(&fps_text, 12, 40, 20, FPS_TEXT_COLOR);

        let f_time: f32 = d.get_frame_time() as f32;
      
        // Draw ants
        for i in 0..ant_list.len() {
            let mut ant: &mut ant::Ant = ant_list.get_mut(i).unwrap();

            ant.position.x += ant.speed * ant.velocity.x * f_time;
            ant.position.y += ant.speed * ant.velocity.y * f_time;
            
            // Trail creation
            trail_list.push(trail::Trail { 
                position: ant.position, 
                color: TRAIL_BASE_COLOR ,
                counter: 255 * 10000
            });

            d.draw_circle(
                ant.position.x as i32, 
                ant.position.y as i32, 
                ant.radius, 
                ANT_COLOR
            );

            ant.check_wall_collision(WIDTH, HEIGHT);
        }

        // Draw tails
        for i in 0..trail_list.len() {
            let mut trail: &mut trail::Trail = match trail_list.get_mut(i) { 
                Some(i) => i,
                None => continue
            };

            trail.counter -= TRAIL_CONSUME_SPEED;
            trail.color.a = (trail.counter / TRAIL_ALPHA_MULTIPLIER) as u8;

            if trail.color.a <= 1 {
                trail_list.remove(i);
                continue;
            }

            trail.color = Color {
                r: trail.color.r,  
                g: trail.color.g,
                b: trail.color.b,
                a: trail.color.a
            };

            d.draw_pixel(
                trail.position.x as i32, 
                trail.position.y as i32,
                trail.color
            );
        }
    }
}