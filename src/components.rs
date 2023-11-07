use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Circle;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct Segment;

#[derive(Component, Clone, Copy)]
pub struct ZIdx(pub f32);

#[derive(Component, Clone, Copy)]
pub struct Collided;

#[derive(Component, Clone)]
pub struct Line {
    pub from: Vec2,
    pub to: Vec2,
    pub color: Color,
}

#[derive(Component, Clone)]
pub struct Arc {
    pub from: Vec2,
    pub center: Vec2,
    pub radius: f32,
    pub angle: f32,
    pub color: Color,
}

#[derive(Component, Clone)]
pub struct Head {
    pub radius: f32,
    pub color: Color,
    pub tail: Option<Entity>
}

pub trait ToPath {
    fn to_path(&self) ->  Path;
}
impl ToPath for Line {
    fn to_path(&self) ->  Path {
        let mut builder = PathBuilder::new();
        builder.move_to(self.from);
        builder.line_to(self.to);
        builder.build()
    }
}

impl ToPath for Arc {
    fn to_path(&self) ->  Path {
        let mut builder = PathBuilder::new();
        builder.move_to(self.from);
        builder.arc(self.center, Vec2::new(self.radius, self.radius), self.angle, 0.0);
        builder.build()
    }
}

impl ToPath for Head {
    fn to_path(&self) ->  Path {
        GeometryBuilder::build_as(&Circle {
            radius: self.radius,
            center: Vec2::ZERO,
        })
    }
}

