extern crate piston_window;

use piston_window::*;

extern crate Physics2D;
extern crate rand;

use rand::Rng;

use Physics2D::physics::Vec2D;
use Physics2D::physics::shapes::Circle;
use Physics2D::physics::shapes::Line;
use Physics2D::physics::Object;
use Physics2D::renderer::Renderable;
use Physics2D::physics::World;

use std::time::Instant;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World", [800,800]).exit_on_esc(true).build().unwrap();
    let wall1 = Line::new(Vec2D::new(0.0, 0.0), Vec2D::new(800.0, 0.0));
    let wall2 = Line::new(Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 800.0));
    let wall3 = Line::new(Vec2D::new(0.0, 800.0), Vec2D::new(800.0, 800.0));
    let wall4 = Line::new(Vec2D::new(800.0, 800.0), Vec2D::new(800.0, 0.0));

    let line1 = Line::new(Vec2D::new(100.0,200.0), Vec2D::new(200.0,100.0));
    let line2 = Line::new(Vec2D::new(700.0,600.0), Vec2D::new(600.0,700.0));
    let line3 = Line::new(Vec2D::new(700.0,200.0), Vec2D::new(600.0,100.0));
    let line4 = Line::new(Vec2D::new(100.0,600.0), Vec2D::new(200.0,700.0));

    let mut world = World::new(0.0, 1.0);
    for i in 0..30 {
        let mut rng = rand::thread_rng();

        let rand_x = rng.gen_range::<f64>(0.0, 800.0);
        let rand_y = rng.gen_range::<f64>(0.0, 800.0);
        let m_noise = rng.gen_range::<f64>(0.0, 10.0);
        let vx_noise = rng.gen_range::<f64>(-7.0, 7.0);
        let vy_noise = rng.gen_range::<f64>(-7.0, 7.0);

        let mut circle = Circle::new(1.0 + m_noise, Vec2D::new(rand_x, rand_y), 10.0 + m_noise);
        circle.set_velocity(&Vec2D::new(vx_noise, vy_noise));
        world.add_object(circle);

    }

    world.add_object(wall1);
    world.add_object(wall2);
    world.add_object(wall3);
    world.add_object(wall4);

    world.add_object(line1);
    world.add_object(line2);
    world.add_object(line3);
    world.add_object(line4);

    while let Some(e) = window.next() {
        let prev_time = Instant::now();
        window.draw_2d(&e, |c, g| {
            clear([1.0,1.0,1.0,1.0], g);

            world.render(&c, g);
            world.update();
        });
        let frame_time = Instant::now().duration_since(prev_time);
        let frame_time_sec: f64 = frame_time.as_secs() as f64;
        let frame_time_sub_nano: f64 = (frame_time.subsec_nanos() as f64) / 1000000000.0;
        let frame_time_sec_total: f64 = frame_time_sec + frame_time_sub_nano;

        let fps = (1.0 / frame_time_sec_total) as u64;
        println!("FPS: {}", fps);
    }
}
