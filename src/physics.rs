use graphics::math::Vec2d;

#[derive(Clone)]
pub enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

#[derive(Clone)]
pub enum CollisionType {
    Kinematic { trigger: bool },
    Static { trigger: bool },
    Dynamic,
}

#[derive(Clone)]
pub enum BodyDef {
    Collide {
        shape: Shape,
        collision_type: CollisionType,
        friction: f64,
        restitution: f64,
    },
    NoCollide,
}

pub enum Gravity {
    Simple {
        strength: f64,
        direction: Vec2d,
    },
    Point {
        strength: f64,
        location: Vec2d,
    },
    None,
}

pub struct World {
    pub gravity: Gravity,
}

pub struct Body {
    pub definition: BodyDef,
    pub velocity: Vec2d,
    pub position: Vec2d,
    pub rotation: f64,
}

impl Body {
    pub fn new(def: BodyDef) -> Body {
        Body {
            definition: def,
            velocity: [0.0; 2],
            position: [0.0; 2],
            rotation: 0.0
        }
    }
}

