use renderer::RenderableObject;

pub mod shapes;
use physics::shapes::*;

use std::any::Any;
use std::collections::HashMap;

//Trait marker signifying object for simulation
pub trait Object {
    fn get_com(&self) -> Vec2D;
    fn set_com(&mut self, com: &Vec2D);

    fn get_mass(&self) -> f64;
    fn set_mass(&mut self, mass: f64);

    fn get_velocity(&self) -> Vec2D;
    fn set_velocity(&mut self, velocity: &Vec2D);

    fn get_friction(&self) -> f64;
    fn set_friction(&mut self, friction_k: f64);

    fn get_static(&self) -> bool;
    fn set_static(&mut self, is_static: bool);

    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);

    fn get_pivot(&self) -> Joint;
    fn set_pivot(&mut self, pivot: Joint);

    fn get_anchor(&self, id: usize) -> Option<&Vec2D>;
    fn get_anchors(&self) -> &HashMap<usize, Vec2D>;
    fn add_anchor(&mut self, id: usize, position: Vec2D);

    fn rotate(&mut self, angle: f64, pivot: Vec2D);

    fn get_i_com(&self) -> f64;

    fn get_ang_velocity(&self) -> f64;
    fn set_ang_velocity(&mut self, ang_velocity: f64);

    fn apply_impulse(&mut self, momentum: &Vec2D, position: &Vec2D, pivot: &Vec2D) {
        // ΔL = r x Δp
        let r = position.sub(&pivot);
        let delta_ang_momentum = r.x * momentum.y - r.y * momentum.x;

        //Parallel Axis Theorem
        let i_pivot = self.get_i_com() + self.get_mass() * pivot.sub(&self.get_com()).mag().powi(2);

        // I * Δѡ = ΔL
        let final_ang_velocity = self.get_ang_velocity() + delta_ang_momentum / i_pivot;
        self.set_ang_velocity(final_ang_velocity);
    }

    fn as_any(&self) -> &Any;
}

pub trait Collidable {
    //Returns whether two bodies has collided
    fn has_collided(&self, other: &RenderableObject) -> bool;

    //Returns vector describing direction of collision(self on other) and the position of the contact point
    fn collision_direction(&self, other: &RenderableObject) -> Option<(Vec2D, Vec2D)>;
}

#[derive(Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

pub struct World {
    pub gravity: f64,
    pub objects: HashMap<usize, Box<RenderableObject>>,
    pub timestep: f64,
    pub next_id: usize,
}

pub struct Joint {
    pub is_dynamic: bool,
    pub position: Vec2D
}

pub struct Link {
    pub object1_id: usize,
    pub object2_id: usize,
    pub anchor1_id: usize,
    pub anchor2_id: usize
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

    pub fn perp(&self) -> Vec2D {
        Vec2D::new(-1.0 * self.y, self.x)
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

impl Joint {
    pub fn new(is_dynamic: bool, position: Vec2D) -> Joint {
        Joint {
            is_dynamic,
            position,
        }
    }
}

impl Clone for Joint {
    fn clone(&self) -> Self {
        Joint::new(self.is_dynamic, self.position.clone())
    }
}

impl World {
    pub fn new(gravity: f64, timestep: f64) -> World {
        World {gravity, objects: HashMap::new(), timestep, next_id: 0}
    }

    pub fn add_object<T: RenderableObject + 'static>(&mut self, object: T) {
        let mut object_id = object.get_id();

        //0 is the default id.
        if object_id != 0 {
            assert!(!self.objects.contains_key(&object_id), "User-defined ID is already taken!");
        }

        //Keep cycling through uuids until there are no hits
        while self.objects.contains_key(&object_id) {
            object_id = self.next_id;
            self.next_id += 1;
        }
        self.objects.insert(object_id, Box::new(object));
    }

    pub fn update(&mut self) {
        let mut keys: Vec<usize> = Vec::with_capacity(self.objects.len());

        for key in self.objects.keys() {
            keys.push(*key);
        }

        //Check for collisions and change trajectories
        for a in 0..self.objects.len() {
            for b in (a+1)..self.objects.len() {
                let i = &keys[a];
                let j = &keys[b];

//                self.objects.get_mut(j).unwrap().rotate(0.01, Vec2D::new(0.0, 0.0));

                let has_collided = self.objects.get(i).unwrap().has_collided(&**self.objects.get(j).unwrap());
                let (collision_direction, contact_point) = match self.objects.get(i).unwrap().collision_direction(&**self.objects.get(j).unwrap()) {
                    Some(v) => v,
                    None => (Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 0.0))
                };

//                if contact_point.x != 0.0 && contact_point.y != 0.0 {
//                    println!("{:?}", contact_point);
//                }


                //Check if colliding objects are fixed/static and perform appropriate collision
                if !self.objects.get_mut(i).unwrap().get_static() && !self.objects.get_mut(j).unwrap().get_static() {
                    //Perform elastic collision if non of the objects are static and have collided
                    if has_collided {
                        let current_m = self.objects.get(i).unwrap().get_mass();
                        let other_m = self.objects.get(j).unwrap().get_mass();

                        //Variables for rotation mechanics
                        let current_pivot = self.objects.get(i).unwrap().get_pivot().position;
                        let current_r = contact_point.sub(&current_pivot);
                        let current_ang_velocity = self.objects.get(i).unwrap().get_ang_velocity();
                        let current_i_pivot = self.objects.get(i).unwrap().get_i_com() + current_m * current_pivot.sub(&self.objects.get(i).unwrap().get_com()).mag().powi(2);

                        let other_pivot = self.objects.get(j).unwrap().get_pivot().position;
                        let other_r = contact_point.sub(&other_pivot);
                        let other_ang_velocity = self.objects.get(j).unwrap().get_ang_velocity();
                        let other_i_pivot = self.objects.get(j).unwrap().get_i_com() + other_m * other_pivot.sub(&self.objects.get(j).unwrap().get_com()).mag().powi(2);

                        //Variables for linear motion
                        let current_velocity = self.objects.get(i).unwrap().get_velocity()
                            .add(&current_r.perp().mult(current_ang_velocity));
                        let current_v_parallel = current_velocity.proj_on(&collision_direction);
                        let current_v_tangent = current_velocity.reject_on(&collision_direction);

                        let other_velocity = self.objects.get(j).unwrap().get_velocity()
                            .add(&other_r.perp().mult(other_ang_velocity));
                        let other_v_parallel = other_velocity.proj_on(&collision_direction);
                        let other_v_tangent = other_velocity.reject_on(&collision_direction);

                        //Velocity part parallel to line of collision experiences elastic collision while tangential part remains constant
                        let current_final_v_parallel = current_v_parallel.mult(current_m)
                            .add(&other_v_parallel.mult(other_m))
                            .sub(&current_v_parallel.mult(other_m))
                            .add(&other_v_parallel.mult(other_m))
                            .mult(1.0/(current_m + other_m));

                        //Velocity of second object after elastic collision
                        let other_final_v_parallel = current_v_parallel.sub(&other_v_parallel).add(&current_final_v_parallel);

                        let current_final_linear_v = current_final_v_parallel.add(&current_v_tangent);
                        let other_final_linear_v = other_final_v_parallel.add(&other_v_tangent);

                        //Apply impulse at contact point for rotation
                        self.objects.get_mut(i).unwrap().apply_impulse(&current_final_linear_v.sub(&current_velocity).mult(0.05 * current_m),
                            &contact_point, &current_pivot
                        );
                        self.objects.get_mut(j).unwrap().apply_impulse(&other_final_linear_v.sub(&other_velocity).mult(0.05 * other_m),
                                                                       &contact_point, &other_pivot
                        );

                        //Objects will spin in place if their pivots are not dynamic
                        //Pivot is dynamically set to the contact point so objects will momentarily pivot around the contact point
                        if self.objects.get(i).unwrap().get_pivot().is_dynamic {
                            let mut new_pivot = self.objects.get(i).unwrap().get_pivot();
                            new_pivot.position = contact_point.clone();

                            self.objects.get_mut(i).unwrap().set_pivot(new_pivot);
                            self.objects.get_mut(i).unwrap().set_velocity(&current_final_linear_v);
                        }
                        if self.objects.get(j).unwrap().get_pivot().is_dynamic {
                            let mut new_pivot = self.objects.get(j).unwrap().get_pivot();
                            new_pivot.position = contact_point.clone();

                            self.objects.get_mut(j).unwrap().set_pivot(new_pivot);
                            self.objects.get_mut(j).unwrap().set_velocity(&other_final_linear_v);
                        }
                    }
                } else {
                    if has_collided {
                        //Make static objects have zero velocity
                        //Make non-static objects reflect at angle of incidence
                        if self.objects.get(i).unwrap().get_static() {
                            self.objects.get_mut(i).unwrap().set_velocity(&Vec2D::new(0.0, 0.0));
                        } else {
                            let current_velocity = self.objects.get(i).unwrap().get_velocity();
                            let new_velocity = current_velocity.proj_on(&collision_direction)
                                                        .mult(-1.0)
                                                        .add(&current_velocity.reject_on(&collision_direction));
                            self.objects.get_mut(i).unwrap().set_velocity(&new_velocity);

                            let pivot = self.objects.get_mut(i).unwrap().get_pivot().position;
                            let current_mass = self.objects.get(j).unwrap().get_mass();
                            self.objects.get_mut(i).unwrap().apply_impulse(&new_velocity.sub(&current_velocity).mult(0.05 * current_mass),
                                                                           &contact_point,
                                                                           &pivot
                            );

                            //Pivot is dynamically set to the contact point so objects will momentarily pivot around the contact point
                            if self.objects.get(i).unwrap().get_pivot().is_dynamic {
                                let mut new_pivot = self.objects.get(i).unwrap().get_pivot();
                                new_pivot.position = contact_point.clone();

                                self.objects.get_mut(i).unwrap().set_pivot(new_pivot);
                            }
                        }

                        if self.objects.get(j).unwrap().get_static() {
                            self.objects.get_mut(j).unwrap().set_velocity(&Vec2D::new(0.0, 0.0));
                        } else {
                            let current_velocity = self.objects.get(j).unwrap().get_velocity();
                            let new_velocity = current_velocity.proj_on(&collision_direction)
                                .mult(-1.0)
                                .add(&current_velocity.reject_on(&collision_direction));
                            self.objects.get_mut(j).unwrap().set_velocity(&new_velocity);

                            let pivot = self.objects.get_mut(j).unwrap().get_pivot().position;
                            let current_mass = self.objects.get(j).unwrap().get_mass();
                            self.objects.get_mut(j).unwrap().apply_impulse(&new_velocity.sub(&current_velocity).mult(0.05 * current_mass),
                                                                           &contact_point,
                                                                           &pivot
                            );

                            //Pivot is dynamically set to the contact point so objects will momentarily pivot around the contact point
                            if self.objects.get(j).unwrap().get_pivot().is_dynamic {
                                let mut new_pivot = self.objects.get(j).unwrap().get_pivot();
                                new_pivot.position = contact_point.clone();

                                self.objects.get_mut(j).unwrap().set_pivot(new_pivot);
                            }
                        }
                    }
                }
            }
        }

        //Apply Frictional Force
        for obj in self.objects.values_mut() {
            //Find magnitude of frictional force and make friction vector
            let friction_k = obj.get_mass() * self.gravity.abs() * obj.get_friction();
            let friction_force = obj.get_velocity().unit().mult(friction_k * self.timestep);

            //Makes sure that friction brings object to rest and not negative velocity
            if friction_force.mag() < obj.get_velocity().mag() {
                let new_velocity = obj.get_velocity().sub(&friction_force);
                obj.set_velocity(&new_velocity);
            } else {
                obj.set_velocity(&Vec2D::new(0.0,0.0));
            }
        }

        //Update locations
        for obj in self.objects.values_mut() {
            let com = obj.get_com();
            let velocity = obj.get_velocity();
            let ang_velocity = obj.get_ang_velocity();
            let pivot = obj.get_pivot().position;

            obj.set_com(&com.add(&velocity.mult(self.timestep)));
            obj.rotate(ang_velocity * self.timestep, pivot);

            //The pivot is dynamically put back to the center of mass after having been positioned at the collision point
            if obj.get_pivot().is_dynamic {
                let mut new_pivot = obj.get_pivot();
                new_pivot.position = obj.get_com();

                obj.set_pivot(new_pivot);
            }
        }
    }
}