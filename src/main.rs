extern crate piston_window;

use piston_window::*;

extern crate Physics2D;
extern crate rand;

use rand::Rng;

use Physics2D::physics::Vec2D;
use Physics2D::physics::shapes::Circle;
use Physics2D::physics::shapes::Line;
use Physics2D::physics::shapes::Group;
use Physics2D::physics::Object;
use Physics2D::renderer::Renderable;
use Physics2D::physics::World;

use std::time::Instant;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World", [800,800]).exit_on_esc(true).build().unwrap();
    let mut world = World::new(0.0, 1.0);

    let mut wall = Line::new(Vec2D::new(200.0, 400.0), Vec2D::new(600.0, 400.0));

    let wall1 = Line::new(Vec2D::new(0.0, 0.0), Vec2D::new(800.0, 0.0));
    let wall2 = Line::new(Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 800.0));
    let wall3 = Line::new(Vec2D::new(0.0, 800.0), Vec2D::new(800.0, 800.0));
    let wall4 = Line::new(Vec2D::new(800.0, 800.0), Vec2D::new(800.0, 0.0));

    let mut ball1 = Circle::new(5.0, Vec2D::new(100.0, 100.0), 10.0);
    let mut ball2 = Circle::new(2.0, Vec2D::new(130.0, 100.0), 10.0);
    let mut ball3 = Circle::new(3.0, Vec2D::new(100.0, 130.0), 10.0);
    let mut ball4 = Circle::new(1.0, Vec2D::new(130.0, 130.0), 10.0);

    let mut ball5 = Circle::new(5.0, Vec2D::new(300.0, 500.0), 10.0);
    let mut ball6 = Circle::new(3.0, Vec2D::new(340.0, 500.0), 10.0);
    let mut ball7 = Circle::new(4.0, Vec2D::new(320.0, 450.0), 10.0);

    let mut ball8 = Circle::new(4.0, Vec2D::new(500.0, 550.0), 30.0);
    ball8.set_velocity(&Vec2D::new(3.0, -1.0));

    let mut group1 = Group::new();
    let mut group2 = Group::new();

    group1.add_object(ball1);
    group1.add_object(ball2);
    group1.add_object(ball3);
    group1.add_object(ball4);

    group2.add_object(ball5);
    group2.add_object(ball6);
    group2.add_object(ball7);

    group1.set_velocity(&Vec2D::new(5.0, 3.0));
    group2.set_velocity(&Vec2D::new(-5.0, 3.0));

    world.add_object(group1);
    world.add_object(wall);
    world.add_object(wall1);
    world.add_object(wall2);
    world.add_object(ball8);
    world.add_object(wall3);
    world.add_object(wall4);
    world.add_object(group2);

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
//        println!("FPS: {}", fps);
    }
}
