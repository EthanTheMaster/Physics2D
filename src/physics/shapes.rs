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
    pub is_static: bool,
}

pub struct Line {
    pub start_point: Vec2D,
    pub end_point: Vec2D,
    pub mass: f64,
    pub velocity: Vec2D,
    pub color: [f32; 4],
    pub friction: f64,
    pub is_static: bool
}

pub struct Group {
    pub objects: Vec<Box<RenderableObject>>,
    pub com: Vec2D,
    pub mass: f64,
    pub velocity: Vec2D,
    pub friction: f64,
    pub is_static: bool
}

impl Circle {
    pub fn new(mass: f64, center: Vec2D, radius: f64) -> Circle{
        Circle{ mass,
                velocity: Vec2D::new(0.0, 0.0),
                center,
                radius,
                color: [0.0, 0.0, 0.0, 1.0],
                friction: 0.0,
                is_static: false
        }
    }
}

impl Line {
    pub fn new(start_point: Vec2D, end_point: Vec2D) -> Line{
        Line {
            start_point,
            end_point,
            mass: 1.0,
            velocity: Vec2D::new(0.0, 0.0),
            color: [0.0, 0.0, 0.0, 1.0],
            friction: 0.0,
            is_static: true,
        }
    }
}

impl Group {
    pub fn new() -> Group {
        Group {
            objects: Vec::new(),
            com: Vec2D::new(0.0, 0.0),
            mass: 0.0,
            velocity: Vec2D::new(0.0, 0.0),
            friction: 0.0,
            is_static: false,
        }
    }

    pub fn add_object(&mut self, object: impl RenderableObject + 'static) {
        self.mass += object.get_mass();
        self.objects.push(Box::new(object));

        //Recalculate COM
        let mut com = Vec2D::new(0.0, 0.0);
        for object in self.objects.iter() {
            com.add(&object.get_com().mult(object.get_mass()));
        }
        com = com.mult(1.0/self.mass);

        self.com = com;
    }

    pub fn create_polygon(points: Vec<Vec2D>, mass: f64) -> Group {
        let mut result = Group::new();
        for i in 0..(points.len() - 1) {
            let mut line = Line::new(points[i].clone(), points[i+1].clone());
            line.set_static(false);
            line.set_mass(mass / (points.len() as f64));
            result.add_object(line);
        }

        return result;
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

    fn get_static(&self) -> bool {
        self.is_static
    }

    fn set_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }
}

impl Object for Line {
    fn get_com(&self) -> Vec2D {
        self.start_point.add(&self.end_point.sub(&self.start_point).mult(0.5))
    }

    fn set_com(&mut self, com: &Vec2D) {
        let translation_vec = com.sub(&self.get_com());

        self.start_point = self.start_point.add(&translation_vec);
        self.end_point = self.end_point.add(&translation_vec);
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

    fn get_static(&self) -> bool {
        self.is_static
    }

    fn set_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl Object for Group {
    fn get_com(&self) -> Vec2D {
        self.com.clone()
    }

    fn set_com(&mut self, com: &Vec2D) {
        let displacement = com.sub(&self.com);
        //Shift all objects in group by the displacement
        for object in self.objects.iter_mut() {
            let obj_current_pos = object.get_com();
            object.set_com(&obj_current_pos.add(&displacement));
        }

        self.com = com.clone();
    }

    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn set_mass(&mut self, mass: f64) {
        //Do nothing ... mass should only changes when adding new objects
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

    fn get_static(&self) -> bool {
        self.is_static
    }

    fn set_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl Collidable for Circle {
    fn has_collided(&self, other: &RenderableObject) -> bool {
        if other.as_any().is::<Circle>() {
            let other: &Circle = other.as_any().downcast_ref::<Circle>().unwrap();
            return self.center.sub(&other.center).mag() < (self.radius + other.radius)
        } else if other.as_any().is::<Line>() {
            let line: &Line = other.as_any().downcast_ref::<Line>().unwrap();
            let line_len = line.end_point.sub(&line.start_point).mag();

            //Distance from center to end point
            let distance1 = line.end_point.sub(&self.center).mag();
            //Distance from center to start point
            let distance2 = line.start_point.sub(&self.center).mag();
            //Distance from center to line
            let distance3 = self.center.sub(&line.start_point)
                                    .reject_on(&line.end_point.sub(&line.start_point)).mag();
            //Closest point on the line(not line segment) from the circle
            let closest_point = line.start_point.add(&self.center.sub(&line.start_point).proj_on(&line.end_point.sub(&line.start_point)));

            if distance1 < self.radius || distance2 < self.radius {
                return true;
            }
            if distance3 < self.radius && closest_point.sub(&line.end_point).mag() < line_len && closest_point.sub(&line.start_point).mag() < line_len {
                return true;
            }

            return false
        } else if other.as_any().is::<Group>() {
            //Use collision detection already implemented for Groups and Circles
            let group: &Group = other.as_any().downcast_ref::<Group>().unwrap();
            return group.has_collided(self as &RenderableObject);
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
                    );
        } else if other.as_any().is::<Line>() {
            let line: &Line = other.as_any().downcast_ref::<Line>().unwrap();
            return Some(
                line.end_point.sub(&line.start_point)
                    .reject_on(&line.end_point.sub(&self.center))
            );
        } else if other.as_any().is::<Group>() {
            //Use collision detection already implemented for Groups and Circles
            let group: &Group = other.as_any().downcast_ref::<Group>().unwrap();
            return group.collision_direction(self as &RenderableObject);
        }

        return None;
    }
}

impl Collidable for Line {
    fn has_collided(&self, other: &RenderableObject) -> bool {
        if other.as_any().is::<Circle>() {
            //Use collision detection already implemented for Circles and Lines
            let circle: &Circle = other.as_any().downcast_ref::<Circle>().unwrap();
            return circle.has_collided(self as &RenderableObject);
        } else if other.as_any().is::<Line>() {
            let line2: &Line = other.as_any().downcast_ref::<Line>().unwrap();

            //Parametrize line1 and line2 and solve for t1 and t2...if t1 and t2 are less than 1 and greater than 0 then there is an intersection
            //Line1: x = x_01 + t_1 * dx_1                Line2: x = x_02 + t_2 * dx_2
            //       y = y_01 + t_1 * dy_1                       y = y_02 + t_2 * dy_2
            let line1_displacement = self.end_point.sub(&self.start_point);
            let line2_displacement = line2.end_point.sub(&line2.start_point);

            //Parallel lines cannot intersect
            if line1_displacement.y / line1_displacement.x == line2_displacement.y / line2_displacement.x {
                return false;
            }

            let t1_solved = (line2_displacement.x*(line2.start_point.y - self.start_point.y) - line2_displacement.y*(line2.start_point.x - self.start_point.x)) / (line2_displacement.x * line1_displacement.y - line1_displacement.x * line2_displacement.y);
            let t2_solved = (line1_displacement.x*(line2.start_point.y - self.start_point.y) - line1_displacement.y*(line2.start_point.x - self.start_point.x)) / (line2_displacement.x * line1_displacement.y - line1_displacement.x * line2_displacement.y);
            return  t1_solved < 1.0 && t1_solved > 0.0 && t2_solved < 1.0 && t2_solved > 0.0;
        } else if other.as_any().is::<Group>() {
            //Use collision detection already implemented for Groups and Lines
            let group: &Group = other.as_any().downcast_ref::<Group>().unwrap();
            return group.has_collided(self as &RenderableObject);
        }

        return false;
    }

    fn collision_direction(&self, other: &RenderableObject) -> Option<Vec2D> {
        if other.as_any().is::<Circle>() {
            //Use collision detection already implemented for Circles and Lines
            let circle: &Circle = other.as_any().downcast_ref::<Circle>().unwrap();
            return circle.collision_direction(self as &RenderableObject);
        } else if other.as_any().is::<Group>() {
            //Use collision detection already implemented for Groups and Lines
            let group: &Group = other.as_any().downcast_ref::<Group>().unwrap();
            return group.collision_direction(self as &RenderableObject);
        }

        //Using line collision checking code
        let line2: &Line = other.as_any().downcast_ref::<Line>().unwrap();

        //Parametrize line1 and line2 and solve for t1 and t2...if t1 and t2 are less than 1 and greater than 0 then there is an intersection
        //Line1: x = x_01 + t_1 * dx_1                Line2: x = x_02 + t_2 * dx_2
        //       y = y_01 + t_1 * dy_1                       y = y_02 + t_2 * dy_2
        let line1_displacement = self.end_point.sub(&self.start_point);
        let line2_displacement = line2.end_point.sub(&line2.start_point);

        //Parallel lines cannot intersect
        if line1_displacement.y / line1_displacement.x == line2_displacement.y / line2_displacement.x {
            return None;
        }

        let t1_solved = (line2_displacement.x*(line2.start_point.y - self.start_point.y) - line2_displacement.y*(line2.start_point.x - self.start_point.x)) / (line2_displacement.x * line1_displacement.y - line1_displacement.x * line2_displacement.y);
        let t2_solved = (line1_displacement.x*(line2.start_point.y - self.start_point.y) - line1_displacement.y*(line2.start_point.x - self.start_point.x)) / (line2_displacement.x * line1_displacement.y - line1_displacement.x * line2_displacement.y);
        if t1_solved < 1.0 && t1_solved > 0.0 && t2_solved < 1.0 && t2_solved > 0.0 {
            //Collision is detected and direction must be created

            //Compare the relative distance(0.0-1.0) between contact point and center of the line
            //Line with the closer relative distance is the reference line for the collision direction
            if (t1_solved - 0.5).abs() < (t2_solved - 0.5).abs() {
                //Use vector normal to line1 as collision direction
                return Some(line1_displacement.perp());
            } else {
                //Use Vector normal to line2 as collision direction
                return Some(line2_displacement.perp());
            }
        }

        return None;
    }
}

impl Collidable for Group {
    fn has_collided(&self, other: &RenderableObject) -> bool {
        if other.as_any().is::<Group>() {
            for object in self.objects.iter() {
                for other in other.as_any().downcast_ref::<Group>().unwrap().objects.iter() {
                    if object.has_collided(&**other) {
                        return true;
                    }
                }
            }
        } else {
            for object in self.objects.iter() {
                if object.has_collided(other) {
                    return true;
                }
            }
        }

        return false;
    }

    fn collision_direction(&self, other: &RenderableObject) -> Option<Vec2D> {
        if other.as_any().is::<Group>() {
            for object in self.objects.iter() {
                for other in other.as_any().downcast_ref::<Group>().unwrap().objects.iter() {
                    if let Some(d) = object.collision_direction(&**other) {
                        return Some(d);
                    }
                }
            }
        } else {
            for object in self.objects.iter() {
                if let Some(d) = object.collision_direction(other) {
                    return Some(d);
                }
            }
        }

        return None;
    }
}