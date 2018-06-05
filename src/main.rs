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
use Physics2D::physics::World;
use Physics2D::renderer::Renderable;
use Physics2D::renderer::Camera;

use std::time::Instant;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World", [800,800]).exit_on_esc(true).build().unwrap();
    let mut world = World::new(0.0, 1.0);

    let mut wall1 = Line::new(Vec2D::new(-15.0, 15.0), Vec2D::new(15.0, 15.0));
    let mut wall2 = Line::new(Vec2D::new(15.0, 15.0), Vec2D::new(15.0, -15.0));
    let mut wall3 = Line::new(Vec2D::new(15.0, -15.0), Vec2D::new(-15.0, -15.0));
    let mut wall4 = Line::new(Vec2D::new(-15.0, -15.0), Vec2D::new(-15.0, 15.0));

    let ball1 = Circle::new(3.0, Vec2D::new(5.0, 5.0), 0.4);
    let ball2 = Circle::new(3.0, Vec2D::new(9.0, 5.0), 0.4);
    let ball3 = Circle::new(3.0, Vec2D::new(7.0, 9.0), 0.4);

    let mut group1 = Group::new();
    group1.add_object(ball1);
    group1.add_object(ball2);
    group1.add_object(ball3);

    for i in 0..10 {
        let mut rng = rand::thread_rng();

        let rand_x = rng.gen_range::<f64>(-10.0, 10.0);
        let rand_y = rng.gen_range::<f64>(-10.0, 10.0);
        let rand_rot = rng.gen_range::<f64>(0.0, 6.2831852);
        let rand_mass = rng.gen_range::<f64>(1.0, 7.0);
        let r = rng.gen_range::<f64>(1.0, 1.5);
        let n = rng.gen_range::<i32>(4, 8);

        let rand_vx = rng.gen_range::<f64>(-0.1, 0.1);
        let rand_vy = rng.gen_range::<f64>(-0.1, 0.1);

        let mut points_polygon: Vec<Vec2D> = Vec::new();
        for i in 0..(n+1) {
            let x: f64 = rand_x + r * (rand_rot + i as f64 * 6.2831852 / n as f64).cos();
            let y: f64 = rand_y +r * (rand_rot + i as f64* 6.2831852 / n as f64).sin();
            points_polygon.push(Vec2D::new(x, y));
        }
        let mut polygon = Group::create_polygon(points_polygon, rand_mass);
        polygon.set_velocity(&Vec2D::new(rand_vx, rand_vy));
        world.add_object(polygon);
    }

    world.add_object(group1);
    world.add_object(wall1);
    world.add_object(wall2);
    world.add_object(wall3);
    world.add_object(wall4);

    while let Some(e) = window.next() {
        let prev_time = Instant::now();
        window.draw_2d(&e, |c, g| {
            clear([1.0,1.0,1.0,1.0], g);
            let camera: Camera = Camera::new(-20.0, 20.0, -20.0, 20.0, c.get_view_size()[0], c.get_view_size()[1]);
            world.render(&c, g, &camera);
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
