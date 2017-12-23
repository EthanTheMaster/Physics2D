use renderer::RenderableObject;

pub mod shapes;
use physics::shapes::*;

use std::any::Any;

//Trait marker signifying object for simulation
pub trait Object {
    fn get_com(&self) -> Vec2D;
    fn set_com(&mut self, com: &Vec2D);

    fn get_mass(&self) -> f64;
    fn set_mass(&mut self, mass: f64);

    fn get_velocity(&self) -> Vec2D;
    fn set_velocity(&mut self, velocity: &Vec2D);

    fn as_any(&self) -> &Any;
}

pub trait Collidable<T: Object> {
    //Returns whether two bodies has collided
    fn has_collided(&self, other: &T) -> bool;

    //Returns vector describing direction of collision(self on other)
    fn collision_direction(&self, other: &T) -> Option<Vec2D>;
}

#[derive(Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

pub struct World {
    pub gravity: f64,
    pub objects: Vec<Box<RenderableObject>>,
    pub timestep: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D{x, y}
    }

    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    pub fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vec2D) -> Vec2D {
        Vec2D::new(self.x - other.x, self.y - other.y)
    }

    pub fn mult(&self, scalar: f64) -> Vec2D {
        Vec2D::new(self.x * scalar, self.y * scalar)
    }

    pub fn dot(&self, other: &Vec2D) -> f64{
        self.x * other.x + self.y * other.y
    }

    pub fn unit(&self) -> Vec2D {
        let mag = self.mag();
        Vec2D::new(self.x / mag, self.y / mag)
    }

    pub fn proj_on(&self, other: &Vec2D) -> Vec2D {
        other.unit().mult(self.dot(other) / other.mag())
    }

    pub fn reject_on(&self, other: &Vec2D) -> Vec2D {
        self.sub(&self.proj_on(&other))
    }
}

impl Clone for Vec2D {
    fn clone(&self) -> Vec2D {
        Vec2D::new(self.x, self.y)
    }

    fn clone_from(&mut self, source: &Vec2D) {
        self.x = source.x;
        self.y = source.y;
    }
}

impl World {
    pub fn new(gravity: f64, timestep: f64) -> World {
        World {gravity, objects: Vec::new(), timestep}
    }

    pub fn add_object<T: RenderableObject + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn update(&mut self) {
        //Check for collisions and change trajectories
        for i in 0..self.objects.len() {
            for j in (i+1)..self.objects.len() {
                let mut has_collided = false;
                let mut collision_direction = Vec2D::new(0.0, 0.0);

                {
                    let current = self.objects[i].as_any().downcast_ref::<Circle>().unwrap();
                    let other = self.objects[j].as_any().downcast_ref::<Circle>().unwrap();

                    has_collided = current.has_collided(&other);

                    collision_direction = match current.collision_direction(&other){
                            Some(v) => v,
                            None => Vec2D::new(0.0,0.0)
                    };
                }

                //Perform elastic collision
                if has_collided {
                    let current_m = self.objects[i].get_mass();
                    let other_m = self.objects[j].get_mass();

                    let mut current_velocity = self.objects[i].get_velocity();
                    let mut other_velocity = self.objects[j].get_velocity();

                    let current_final_v = current_velocity.mult(current_m)
                                                            .add(&other_velocity.mult(other_m))
                                                            .sub(&current_velocity.mult(other_m))
                                                            .add(&other_velocity.mult(other_m))
                                                            .mult(1.0/(current_m + other_m));
                    let other_final_v = current_velocity.sub(&other_velocity).add(&current_final_v);

                    self.objects[i].set_velocity(&current_final_v);
                    self.objects[j].set_velocity(&other_final_v);
                }
            }
        }

        //Update locations
        for obj in self.objects.iter_mut() {
            let com = obj.get_com();
            let velocity = obj.get_velocity();

            obj.set_com(&com.add(&velocity.mult(self.timestep)));
        }
    }
}