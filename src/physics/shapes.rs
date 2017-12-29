use physics::Vec2D;
use physics::Object;
use physics::Collidable;

use renderer::RenderableObject;

use std::any::Any;

pub struct Circle {
    pub mass: f64,
    pub velocity: Vec2D,
    pub center: Vec2D,
    pub radius: f64,
    pub color: [f32; 4],
    pub friction: f64,
}

impl Circle {
    pub fn new(mass: f64, center: Vec2D, radius: f64) -> Circle{
        Circle{ mass,
                velocity: Vec2D::new(0.0, 0.0),
                center,
                radius,
                color: [0.0, 0.0, 0.0, 1.0],
                friction: 0.0
        }
    }
}

impl Object for Circle{
    fn get_com(&self) -> Vec2D {
        self.center.clone()
    }

    fn set_com(&mut self, com: &Vec2D) {
        self.center = com.clone();
    }

    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    fn get_velocity(&self) -> Vec2D {
        self.velocity.clone()
    }

    fn set_velocity(&mut self, velocity: &Vec2D) {
        self.velocity = velocity.clone();
    }

    fn get_friction(&self) -> f64 {
        self.friction
    }

    fn set_friction(&mut self, friction_k: f64) {
        self.friction = friction_k;
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl Collidable for Circle {
    fn has_collided(&self, other: &RenderableObject) -> bool {
        if other.as_any().is::<Circle>() {
            let other = other.as_any().downcast_ref::<Circle>().unwrap();
            return self.center.sub(&other.center).mag() < (self.radius + other.radius)
        }

        return false;
    }

    fn collision_direction(&self, other: &RenderableObject) -> Option<Vec2D> {
        if !self.has_collided(other) {
            return None;
        }

        if other.as_any().is::<Circle>() {
            let other = other.as_any().downcast_ref::<Circle>().unwrap();
            return Some(
                        Vec2D::new(other.center.x - self.center.x, other.center.y - self.center.y)
                    )
        }

        return None;
    }
}