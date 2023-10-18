use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(FixedTime::new_from_secs(TIMESTEP))
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, (
            update_acceleration,
            update_positions,
            update_collisions,
            update_paths_system,
        ).chain())
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Clone)]
struct Position(Vec2);

#[derive(Component, Clone)]
struct Velocity(Vec2);

#[derive(Component, Clone)]
struct Radius(f32);

#[derive(Component)]
struct Segment;

#[derive(Component, Clone)]
struct Line {
    from: Vec2,
    to: Vec2,
}

impl Line {
    fn to_path(&self) -> Path {
        let mut builder = PathBuilder::new();
        builder.move_to(self.from);
        builder.line_to(self.to);
        builder.build()
    }
}

#[derive(Component, Clone)]
struct Arc {
    from: Vec2,
    center: Vec2,
    radius: f32,
    angle: f32,
}

impl Arc {
    fn to_path(&self) -> Path {
        let mut builder = PathBuilder::new();
        builder.move_to(self.from);
        builder.arc(self.center, Vec2::new(self.radius, self.radius), self.angle, 0.0);
        builder.build()
    }
}

#[derive(Component, Clone)]
struct Head {
    pos: Vec2,
    tail: Option<Entity>
}

impl Head {
    fn to_path(&self) -> Path {
        GeometryBuilder::build_as(&Circle {
            radius: 10.,
            center: self.pos,
        })
    }
}


const RADIUS: f32 = 10.0;
const VEL: f32 = 20.0;
const TIMESTEP: f32 = 0.100;
const EPSILON: f32 = 0.00001;
fn update_acceleration(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Radius), With<Player>>) {
    let mut r  = query.get_single_mut().unwrap();
    if keys.pressed(KeyCode::Left) {
        *r = Radius(RADIUS);
    } else if keys.pressed(KeyCode::Right) {
        *r = Radius(-RADIUS);
    } else {
        *r = Radius(f32::INFINITY);
    }
}

fn update_collisions(mut query: Query<&Segment>) {
    for c1 in query.iter() {
        for c2 in query.iter() {
        }
    }
}

fn update_positions(
    mut query: Query<(&mut Position, &mut Velocity, &mut Head, &Radius, )>,
    mut q_lines: Query<&mut Line>,
    mut q_arcs: Query<&mut Arc>,
    mut commands: Commands,
) {
    for (mut p, mut v, mut h, r,) in query.iter_mut() {
        let dt = TIMESTEP;
        if r.0.is_infinite() {
            *p = Position(p.0 + v.0*dt);
            if let Some(mut line) = h.tail.and_then(|t| q_lines.get_mut(t).ok()) {
                line.to = p.0;
            } else {
                h.tail = Some(commands.spawn((
                    Line {
                        from: h.pos,
                        to: p.0,
                    },
                )).id());
            }
        } else {
            let radius_vec = -v.0.perp().normalize();
            let center = p.0 - radius_vec*r.0;
            let rho = VEL/r.0;
            let angle = rho*dt;
            let new_radius_vec = Vec2::from_angle(angle).rotate(radius_vec);
            let delta = (new_radius_vec - radius_vec)*r.0;
            *p = Position(p.0 + delta);
            *v = Velocity(new_radius_vec.perp()*v.0.length());
            if let Some(mut arc) = h.tail.and_then(|t| q_arcs.get_mut(t).ok()).filter(|arc| (arc.center - center).length() < EPSILON)  {
                arc.angle += angle;
            } else {
                h.tail = Some(commands.spawn((
                    Arc {
                        from: h.pos,
                        center,
                        radius: r.0.abs(),
                        angle,
                    },
                )).id());
            }
        }
        h.pos = p.0;
    }
}

fn update_paths_system(
    q_lines: Query<(Entity, &Line,), Changed<Line>>,
    q_arcs: Query<(Entity, &Arc,), Changed<Arc>>,
    q_heads: Query<(Entity, &Head,), Changed<Head>>,
    mut commands: Commands,
) {
    for (e, l, ) in q_lines.iter() {
        commands.entity(e).insert((
            ShapeBundle {
                path: l.to_path(),
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
        ));
    }
    for (e, a, ) in q_arcs.iter() {
        commands.entity(e).insert((
            ShapeBundle {
                path: a.to_path(),
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
        ));
    }
    for (e, h, ) in q_heads.iter() {
        commands.entity(e).insert((
            ShapeBundle {
                path: h.to_path(),
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
        ));
    }
}

fn setup_system(mut commands: Commands) {
    let head = Head {
        pos: Vec2::ZERO,
        tail: None,
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Name::new("Curve 1".to_owned()),
        SpatialBundle {
            ..Default::default()
        },
        Player,
        Position(head.pos),
        Velocity(Vec2::new(VEL, 0.0)),
        Radius(f32::INFINITY),
        head,
    ));
}
