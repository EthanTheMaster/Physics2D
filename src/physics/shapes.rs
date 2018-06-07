use physics::Vec2D;
use physics::Object;
use physics::Joint;
use physics::Collidable;

use renderer::RenderableObject;

use std::any::Any;
use std::collections::HashMap;

pub struct Circle {
    pub mass: f64,
    pub velocity: Vec2D,
    pub center: Vec2D,
    pub radius: f64,
    pub color: [f32; 4],
    pub friction: f64,
    pub is_static: bool,
    pub id: usize,
    pub pivot: Joint,
    pub anchors: HashMap<usize, Vec2D>,
    pub ang_velocity: f64
}

pub struct Line {
    pub start_point: Vec2D,
    pub end_point: Vec2D,
    pub mass: f64,
    pub velocity: Vec2D,
    pub color: [f32; 4],
    pub friction: f64,
    pub is_static: bool,
    pub id: usize,
    pub pivot: Joint,
    pub anchors: HashMap<usize, Vec2D>,
    pub ang_velocity: f64
}

pub struct Group {
    pub objects: Vec<Box<RenderableObject>>,
    pub com: Vec2D,
    pub mass: f64,
    pub velocity: Vec2D,
    pub friction: f64,
    pub is_static: bool,
    pub id: usize,
    pub pivot: Joint,
    pub anchors: HashMap<usize, Vec2D>,
    pub ang_velocity: f64
}

impl Circle {
    pub fn new(mass: f64, center: Vec2D, radius: f64) -> Circle{
        Circle{ mass,
                velocity: Vec2D::new(0.0, 0.0),
                center: center.clone(),
                radius,
                color: [0.0, 0.0, 0.0, 1.0],
                friction: 0.0,
                is_static: false,
                id: 0,
                pivot: Joint::new(true, center),
                anchors: HashMap::new(),
                ang_velocity: 0.0
        }
    }
}

impl Line {
    pub fn new(start_point: Vec2D, end_point: Vec2D) -> Line{
        let com = end_point.sub(&start_point).mult(0.5).add(&start_point);
        Line {
            start_point,
            end_point,
            mass: 1.0,
            velocity: Vec2D::new(0.0, 0.0),
            color: [0.0, 0.0, 0.0, 1.0],
            friction: 0.0,
            is_static: true,
            id: 0,
            pivot: Joint::new(true, com),
            anchors: HashMap::new(),
            ang_velocity: 0.0
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
            id: 0,
            pivot: Joint::new(true, Vec2D::new(0.0, 0.0)),
            anchors: HashMap::new(),
            ang_velocity: 0.0
        }
    }

    pub fn add_object(&mut self, object: impl RenderableObject + 'static) {
        self.mass += object.get_mass();
        self.objects.push(Box::new(object));

        //Recalculate COM
        let mut com = Vec2D::new(0.0, 0.0);
        for object in self.objects.iter() {
            com = com.add(&object.get_com().mult(object.get_mass()));

            for (id, anchor) in object.get_anchors() {
                self.anchors.insert(*id, anchor.clone());
            }
        }
        com = com.mult(1.0/self.mass);

        self.com = com;

        if self.pivot.is_dynamic {
            self.pivot.position = self.get_com();
        }
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

fn rotate(angle: f64, vector: &Vec2D, pivot: &Vec2D) -> Vec2D {
    let shifted_vector = vector.sub(pivot);
    let shifted_rotated = Vec2D::new(
        shifted_vector.dot(&Vec2D::new(angle.cos(), -1.0 * angle.sin())),
        shifted_vector.dot(&Vec2D::new(angle.sin(), angle.cos()))
    );

    return pivot.add(&shifted_rotated);
}

impl Object for Circle{
    fn get_com(&self) -> Vec2D {
        self.center.clone()
    }

    fn set_com(&mut self, com: &Vec2D) {
        let translation = com.sub(&self.get_com());
        for anchor in self.anchors.values_mut() {
            *anchor = anchor.add(&translation);
        }

        self.center = com.clone();

        if self.pivot.is_dynamic {
            self.pivot.position = self.pivot.position.add(&translation);
        }
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

    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_pivot(&self) -> Joint {
        self.pivot.clone()
    }

    fn set_pivot(&mut self, pivot: Joint) {
        self.pivot = pivot;
    }

    fn get_anchor(&self, id: usize) -> Option<&Vec2D> {
        self.anchors.get(&id)
    }

    fn add_anchor(&mut self, id: usize, position: Vec2D) {
        self.anchors.insert(id, position);
    }

    fn rotate(&mut self, angle: f64, pivot: Vec2D) {
        self.center = rotate(angle, &self.center, &pivot);

        if self.pivot.is_dynamic {
            self.pivot.position = rotate(angle, &self.pivot.position, &pivot);
        }
        for anchor in self.anchors.values_mut() {
            *anchor = rotate(angle, anchor, &pivot);
        }
    }

    fn get_anchors(&self) -> &HashMap<usize, Vec2D> {
        &self.anchors
    }

    fn get_i_com(&self) -> f64 {
        return 0.5 * self.mass * self.radius.powi(2);
    }

    fn get_ang_velocity(&self) -> f64 {
        self.ang_velocity
    }

    fn set_ang_velocity(&mut self, ang_velocity: f64) {
        self.ang_velocity = ang_velocity
    }
}

impl Object for Line {
    fn get_com(&self) -> Vec2D {
        self.start_point.add(&self.end_point.sub(&self.start_point).mult(0.5))
    }

    fn set_com(&mut self, com: &Vec2D) {
        let translation_vec = com.sub(&self.get_com());

        for anchor in self.anchors.values_mut() {
            *anchor = anchor.add(&translation_vec);
        }

        self.start_point = self.start_point.add(&translation_vec);
        self.end_point = self.end_point.add(&translation_vec);

        if self.pivot.is_dynamic {
            self.pivot.position = self.pivot.position.add(&translation_vec);
        }
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

    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_pivot(&self) -> Joint {
        self.pivot.clone()
    }

    fn set_pivot(&mut self, pivot: Joint) {
        self.pivot = pivot;
    }

    fn get_anchor(&self, id: usize) -> Option<&Vec2D> {
        self.anchors.get(&id)
    }

    fn add_anchor(&mut self, id: usize, position: Vec2D) {
        self.anchors.insert(id, position);
    }

    fn rotate(&mut self, angle: f64, pivot: Vec2D) {
        self.start_point = rotate(angle, &self.start_point, &pivot);
        self.end_point = rotate(angle, &self.end_point, &pivot);

        if self.pivot.is_dynamic {
            self.pivot.position = rotate(angle, &self.pivot.position, &pivot);
        }
        for anchor in self.anchors.values_mut() {
            *anchor = rotate(angle, anchor, &pivot);
        }
    }

    fn get_anchors(&self) -> &HashMap<usize, Vec2D> {
        &self.anchors
    }

    fn get_i_com(&self) -> f64 {
        return (1.0/12.0) * self.mass * self.end_point.sub(&self.start_point).mag().powi(2);
    }

    fn get_ang_velocity(&self) -> f64 {
        self.ang_velocity
    }

    fn set_ang_velocity(&mut self, ang_velocity: f64) {
        self.ang_velocity = ang_velocity
    }
}

impl Object for Group {
    fn get_com(&self) -> Vec2D {
        self.com.clone()
    }

    fn set_com(&mut self, com: &Vec2D) {
        let displacement = com.sub(&self.com);

        for anchor in self.anchors.values_mut() {
            *anchor = anchor.add(&displacement);
        }

        //Shift all objects in group by the displacement
        for object in self.objects.iter_mut() {
            let obj_current_pos = object.get_com();
            object.set_com(&obj_current_pos.add(&displacement));
        }

        self.com = com.clone();

        if self.pivot.is_dynamic {
            self.pivot.position = self.pivot.position.add(&displacement);
        }
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

    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_pivot(&self) -> Joint {
        self.pivot.clone()
    }

    fn set_pivot(&mut self, pivot: Joint) {
        self.pivot = pivot;
    }

    fn get_anchor(&self, id: usize) -> Option<&Vec2D> {
        self.anchors.get(&id)
    }

    fn add_anchor(&mut self, id: usize, position: Vec2D) {
        self.anchors.insert(id, position);
    }

    fn rotate(&mut self, angle: f64, pivot: Vec2D) {
        for object in self.objects.iter_mut() {
            object.rotate(angle, pivot.clone());
        }

        self.com = rotate(angle, &self.com, &pivot);

        if self.pivot.is_dynamic {
            self.pivot.position = rotate(angle, &self.pivot.position, &pivot);
        }

        for anchor in self.anchors.values_mut() {
            *anchor = rotate(angle, anchor, &pivot);
        }
    }

    fn get_anchors(&self) -> &HashMap<usize, Vec2D> {
        &self.anchors
    }

    fn get_i_com(&self) -> f64 {
        let mut total_i = 0.0;

        for object in self.objects.iter() {
            //Parallel Axis Theorem
            let h = object.get_com().sub(&self.com).mag();
            total_i += object.get_i_com() + object.get_mass() * h.powi(2);
        }

        return total_i;
    }

    fn get_ang_velocity(&self) -> f64 {
        self.ang_velocity
    }

    fn set_ang_velocity(&mut self, ang_velocity: f64) {
        self.ang_velocity = ang_velocity;
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

    fn collision_direction(&self, other: &RenderableObject) -> Option<(Vec2D, Vec2D)> {
        if !self.has_collided(other) {
            return None;
        }

        if other.as_any().is::<Circle>() {
            let other = other.as_any().downcast_ref::<Circle>().unwrap();
            return Some((
                        Vec2D::new(other.center.x - self.center.x, other.center.y - self.center.y),
                        other.center.sub(&self.center).mult(self.radius/(self.radius + other.radius)).add(&self.center)
            ));
        } else if other.as_any().is::<Line>() {
            let line: &Line = other.as_any().downcast_ref::<Line>().unwrap();
            return Some((
                line.end_point.sub(&line.start_point).perp(),
                self.center.sub(&line.start_point).proj_on(&line.end_point.sub(&line.start_point)).add(&line.start_point)
            ));
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

    fn collision_direction(&self, other: &RenderableObject) -> Option<(Vec2D, Vec2D)> {
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
                return Some((
                    line1_displacement.perp(),
                    self.start_point.add(&line1_displacement.mult(t1_solved))
                ));
            } else {
                //Use Vector normal to line2 as collision direction
                return Some((
                    line2_displacement.perp(),
                    line2.start_point.add(&line2_displacement.mult(t2_solved))
                ));
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

    fn collision_direction(&self, other: &RenderableObject) -> Option<(Vec2D, Vec2D)> {
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