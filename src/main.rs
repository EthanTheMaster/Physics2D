extern crate piston_window;

use piston_window::*;

extern crate Physics2D;
use Physics2D::physics::Vec2D;
use Physics2D::physics::shapes::Circle;
use Physics2D::physics::Object;
use Physics2D::renderer::Renderable;
use Physics2D::physics::World;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World", [500,500]).exit_on_esc(true).build().unwrap();
    let mut c1 = Circle::new(1.0, Vec2D::new(50.0,50.0), 15.0);
    let mut c2 = Circle::new(1.0, Vec2D::new(200.0,200.0), 15.0);
    let mut c3 = Circle::new(10.0, Vec2D::new(425.0,350.0), 40.0);

    c1.color = [1.0,0.0,0.0,1.0];
    c2.color = [0.0,1.0,0.0,1.0];
    c3.color = [0.0,0.0,1.0,1.0];

    c1.set_velocity(&Vec2D::new(5.0,5.0));
    c2.set_velocity(&Vec2D::new(0.25,0.25));

    let mut world = World::new(0.0, 0.25);
    world.add_object(c1);
    world.add_object(c2);
    world.add_object(c3);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0,1.0,1.0,1.0], g);

            world.render(&c, g);
            world.update();
        });
    }
}
