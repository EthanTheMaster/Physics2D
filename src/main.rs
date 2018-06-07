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
    let mut world = World::new(0.0, 0.5);

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
    group1.set_velocity(&Vec2D::new(0.1, 0.2));

    let ball4 = Circle::new(2.0, Vec2D::new(1.0, 1.0), 0.4);
    let mut line = Line::new(Vec2D::new(1.0, 1.0), Vec2D::new(1.0, -5.0));
    line.set_mass(7.0);
    line.set_static(false);

    let mut group2 = Group::new();
    group2.add_object(ball4);
    group2.add_object(line);

    let mut new_joint = group2.get_pivot();
    new_joint.is_dynamic = false;
    group2.set_pivot(new_joint);

    for i in 0..5 {
        let mut rng = rand::thread_rng();

        let rand_x = rng.gen_range::<f64>(-10.0, 10.0);
        let rand_y = rng.gen_range::<f64>(-10.0, 10.0);
        let rand_rot = rng.gen_range::<f64>(0.0, 6.2831852);
        let rand_mass = rng.gen_range::<f64>(1.0, 7.0);
        let r = rng.gen_range::<f64>(1.0, 1.5);
        let n = rng.gen_range::<i32>(3, 5);

        let rand_vx = rng.gen_range::<f64>(-0.1, 0.1);
        let rand_vy = rng.gen_range::<f64>(-0.1, 0.1);

        let mut points_polygon: Vec<Vec2D> = Vec::new();
        let mut polygon = Group::new();
        for i in 0..(n+1) {
            let x: f64 = rand_x + r * (rand_rot + i as f64 * 6.2831852 / n as f64).cos();
            let y: f64 = rand_y +r * (rand_rot + i as f64* 6.2831852 / n as f64).sin();
//            points_polygon.push(Vec2D::new(x, y));
            polygon.add_object(Circle::new(rand_mass, Vec2D::new(x, y), 0.3));
        }
//        let mut polygon = Group::create_polygon(points_polygon, 4.0);
        polygon.set_velocity(&Vec2D::new(rand_vx, rand_vy));

        let mut new_joint = polygon.get_pivot();
        new_joint.is_dynamic = rng.gen_weighted_bool(2);

//        polygon.set_pivot(new_joint);
        world.add_object(polygon);
    }

    world.add_object(group1);
    world.add_object(wall1);
    world.add_object(wall2);
    world.add_object(wall3);
    world.add_object(wall4);
//    world.add_object(group2);

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
