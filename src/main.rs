extern crate piston_window;

use piston_window::*;

extern crate Physics2D;
use Physics2D::physics::Vec2D;
use Physics2D::physics::shapes::Circle;
use Physics2D::physics::shapes::Line;
use Physics2D::physics::Object;
use Physics2D::renderer::Renderable;
use Physics2D::physics::World;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World", [800,800]).exit_on_esc(true).build().unwrap();
    let mut c1 = Circle::new(1.0, Vec2D::new(50.0,50.0), 15.0);
    let mut c2 = Circle::new(1.0, Vec2D::new(200.0,200.0), 15.0);
    let mut c3 = Circle::new(5.0, Vec2D::new(425.0,350.0), 40.0);

    let mut l1 = Line::new(Vec2D::new(0.0, 600.0), Vec2D::new(800.0, 500.0));
    let mut l2 = Line::new(Vec2D::new(600.0, 200.0), Vec2D::new(700.0, 300.0));
    let mut l3 = Line::new(Vec2D::new(600.0, 800.0), Vec2D::new(800.0, 800.0));

    c1.color = [1.0,0.0,0.0,1.0];
    c2.color = [0.0,1.0,0.0,1.0];
    c3.color = [0.0,0.0,1.0,1.0];

    c2.set_velocity(&Vec2D::new(-3.2,-3.2));
    l3.set_velocity(&Vec2D::new(0.0,-0.5));

    c1.set_static(true);
    l3.set_static(false);

    c1.set_friction(0.001);
    c2.set_friction(0.0);
    c3.set_friction(0.00005);

    let mut world = World::new(9.8, 1.0);
    world.add_object(c1);
    world.add_object(c2);
    world.add_object(c3);

    world.add_object(l1);
    world.add_object(l2);
    world.add_object(l3);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0,1.0,1.0,1.0], g);

            world.render(&c, g);
            world.update();
        });
    }
}
