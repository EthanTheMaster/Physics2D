use piston_window::*;

use physics::shapes::Circle;
use physics::World;
use physics::Object;
use physics::Collidable;

pub trait Renderable {
    fn render(&self, context: &Context, graphics: &mut G2d);
}

pub trait RenderableObject: Renderable + Object + Collidable {}

impl Renderable for Circle {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        graphics.ellipse(&Ellipse::new(self.color),
                         [self.center.x - self.radius, self.center.y - self.radius, 2.0*self.radius, 2.0*self.radius],
                            &context.draw_state, context.transform);
    }
}

impl RenderableObject for Circle {}

impl Renderable for World {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        for obj in self.objects.iter() {
            obj.render(context, graphics);
        }
    }
}