use piston_window::*;

use physics::shapes::Circle;
use physics::shapes::Line;
use physics::shapes::Group;
use physics::World;
use physics::Object;
use physics::Collidable;

use piston_window::Line as GLine;

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

impl Renderable for Line {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        graphics.line(&GLine::new(self.color, 1.0),
                        [self.start_point.x, self.start_point.y, self.end_point.x, self.end_point.y],
                            &context.draw_state, context.transform);
    }
}

impl Renderable for Group {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        for obj in self.objects.iter() {
            obj.render(context, graphics);
        }
    }
}

impl RenderableObject for Circle {}
impl RenderableObject for Line {}
impl RenderableObject for Group {}

impl Renderable for World {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        for obj in self.objects.iter() {
            obj.render(context, graphics);
        }
    }
}