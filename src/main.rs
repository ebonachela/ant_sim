use raylib::prelude::*;
use rand::Rng;

mod ant;
mod trail;
mod food;

// Game constants
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 960;
const TARGET_FPS: u32 = 60;
const FPS_TEXT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;

// Ants constants
const ANT_SPEED: f32 = 100.0;
const ANT_RADIUS: f32 = 1.0;
const ANT_COUNT: i32 = 50;
const ANT_COLOR: Color = Color::WHITE;
const ANT_DETECT_RADIUS: f32 = 10.0;

// Trail constants
const TRAIL_BASE_COLOR: Color = Color::YELLOW;
const TRAIL_ALPHA_MULTIPLIER: i32 = 10000;
const TRAIL_CONSUME_SPEED: i32 = 10000;
const TRAIL_CHECK_RADIUS: f32 = 10.0;
const TRAIL_DEFAULT_PROB: f32 = 0.1; // 1%
const TRAIL_INC_PROB: f32 = 0.01;

// Food constants
const FOOD_RADIUS: f32 = 10.0;
const FOOD_START_COUNT: i32 = 1000;
const FOOD_COLOR: Color = Color::RED;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Ant Simulation")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut trail_list: Vec<trail::Trail> = Vec::new();
    let mut ant_list: Vec<ant::Ant> = Vec::new();
    let mut target_list: Vec<food::TargetType> = Vec::new();

    let food_location: food::TargetType = food::TargetType::Food {
        position: Vector2 {
            x: 100.0,
            y: 100.0
        },
        radius: FOOD_RADIUS,
        count: FOOD_START_COUNT,
        color: FOOD_COLOR
    };
    let base_location: food::TargetType = food::TargetType::Base {
        position: Vector2 {
            x: (WIDTH / 2) as f32,
            y: (HEIGHT / 2) as f32
        },
        radius: FOOD_RADIUS,
        color: Color::BLUE
    };

    target_list.push(food_location);
    target_list.push(base_location);
    
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

    let mut trail_counter: i32 = 10;
  
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

        // Draw food and base
        for i in 0..target_list.len(){
            match target_list.get(i).unwrap() {
                food::TargetType::Food {position, radius, color, count: _} => {
                    d.draw_circle(
                        position.x as i32, 
                        position.y as i32, 
                        *radius, 
                        color
                    );
                },
                food::TargetType::Base {position, radius, color} => {
                    d.draw_circle(
                        position.x as i32, 
                        position.y as i32, 
                        *radius, 
                        color
                    );
                }
            };
        }

        let f_time: f32 = d.get_frame_time() as f32;

        trail_counter -= 1;

        let mut should_add_tail: bool = false;

        if trail_counter <= 0 {
            should_add_tail = true;
            trail_counter = 10;
        }
     
        // Draw ants
        for i in 0..ant_list.len() {
            let mut ant: &mut ant::Ant = ant_list.get_mut(i).unwrap();

            ant.position.x += ant.speed * ant.velocity.x * f_time;
            ant.position.y += ant.speed * ant.velocity.y * f_time;
            
            if should_add_tail {
                // Trail creation
                let new_trail: trail::Trail = trail::Trail { 
                    position: ant.position, 
                    color: TRAIL_BASE_COLOR,
                    counter: 255 * 10000,
                    prob_to_follow: TRAIL_DEFAULT_PROB
                };

                trail_list.push(new_trail);

                trail_counter = 10;

                new_trail
                    .get_prob_to_follow(&mut trail_list, TRAIL_CHECK_RADIUS, TRAIL_INC_PROB);
            }

            d.draw_circle(
                ant.position.x as i32, 
                ant.position.y as i32, 
                ant.radius, 
                ANT_COLOR
            );

            ant.check_wall_collision(WIDTH, HEIGHT);
            ant.check_close_trails(trail_list.clone(), ANT_DETECT_RADIUS);
        }

        // Draw trails
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