use piston_window::*;

use physics::shapes::Circle;
use physics::shapes::Line;
use physics::shapes::Group;
use physics::Vec2D;
use physics::World;
use physics::Object;
use physics::Collidable;

use piston_window::Line as GLine;

pub struct Camera {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub screen_width: f64,
    pub screen_height: f64
}

impl Camera {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64, screen_width: f64, screen_height: f64) -> Camera {
        Camera {
            x_min,
            x_max,
            y_min,
            y_max,
            screen_width,
            screen_height,
        }
    }

    pub fn cartesian(&self, screen_pos: &Vec2D) -> Vec2D {
        let x = (self.x_max - self.x_min) / self.screen_width * screen_pos.x + self.x_min;
        let y = (self.y_min - self.y_max) / self.screen_height * screen_pos.y + self.y_max;

        Vec2D::new(x, y)
    }

    pub fn screen(&self, cartesian_pos: &Vec2D) -> Vec2D {
        let x = self.screen_width / (self.x_max - self.x_min) * (cartesian_pos.x - self.x_min);
        let y = -1.0 * self.screen_height / (self.y_max - self.y_min) * (cartesian_pos.y - self.y_max);

        Vec2D::new(x, y)
    }
}

pub trait Renderable {
    fn render(&self, context: &Context, graphics: &mut G2d, camera: &Camera);
}

pub trait RenderableObject: Renderable + Object + Collidable {}

impl Renderable for Circle {
    fn render(&self, context: &Context, graphics: &mut G2d, camera: &Camera) {
        let center = camera.screen(&self.center);
        let radius_x = camera.screen_width/(camera.x_max - camera.x_min) * self.radius;
        let radius_y = camera.screen_height/(camera.y_max - camera.y_min) * self.radius;
        graphics.ellipse(&Ellipse::new(self.color),
                         [center.x - radius_x, center.y - radius_y, 2.0*radius_x, 2.0*radius_y],
                            &context.draw_state, context.transform);

        let com = camera.screen(&self.get_com());
        graphics.ellipse(&Ellipse::new([1.0,0.0,0.0,1.0]),
                         [com.x - 2.5, com.y - 2.5, 5.0, 5.0],
                         &context.draw_state, context.transform);
        let pivot = camera.screen(&self.pivot.position);
        graphics.ellipse(&Ellipse::new([0.0,1.0,0.0,1.0]),
                         [pivot.x - 2.5, pivot.y - 2.5, 5.0, 5.0],
                         &context.draw_state, context.transform);

    }
}

impl Renderable for Line {
    fn render(&self, context: &Context, graphics: &mut G2d, camera: &Camera) {
        let start_point = camera.screen(&self.start_point);
        let end_point = camera.screen(&self.end_point);
        graphics.line(&GLine::new(self.color, 1.0),
                        [start_point.x, start_point.y, end_point.x, end_point.y],
                            &context.draw_state, context.transform);

        let com = camera.screen(&self.get_com());
        graphics.ellipse(&Ellipse::new([1.0,0.0,0.0,1.0]),
                         [com.x - 2.5, com.y - 2.5, 5.0, 5.0],
                         &context.draw_state, context.transform);
        let pivot = camera.screen(&self.pivot.position);
        graphics.ellipse(&Ellipse::new([0.0,1.0,0.0,1.0]),
                         [pivot.x - 2.5, pivot.y - 2.5, 5.0, 5.0],
                         &context.draw_state, context.transform);
    }
}

impl Renderable for Group {
    fn render(&self, context: &Context, graphics: &mut G2d, camera: &Camera) {
        for obj in self.objects.iter() {
            obj.render(context, graphics, camera);
        }

        let com = camera.screen(&self.get_com());
        graphics.ellipse(&Ellipse::new([1.0,0.0,0.0,1.0]),
                         [com.x - 2.5, com.y - 2.5, 5.0, 5.0],
                         &context.draw_state, context.transform);
        let pivot = camera.screen(&self.pivot.position);
        graphics.ellipse(&Ellipse::new([0.0,1.0,0.0,1.0]),
                         [pivot.x - 2.5, pivot.y - 2.5, 5.0, 5.0],
                         &context.draw_state, context.transform);
    }
}

impl RenderableObject for Circle {}
impl RenderableObject for Line {}
impl RenderableObject for Group {}

impl Renderable for World {
    fn render(&self, context: &Context, graphics: &mut G2d, camera: &Camera) {
        for obj in self.objects.values() {
            obj.render(context, graphics, camera);
        }
    }
}