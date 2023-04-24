use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(FixedTime::new_from_secs(TIMESTEP))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_systems((
            update_acceleration,
            update_positions,
            update_collisions,
            update_paths_system,
        ).chain().in_schedule(CoreSchedule::FixedUpdate))
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

#[derive(Clone)]
enum CurveSegment {
    Line {
        to: Vec2,
    },
    Circle {
        center: Vec2,
        radius: f32,
        angle: f32,
    }
}
impl CurveSegment {
    fn is_line(&self) -> bool {
        match self {
            CurveSegment::Line { .. } => true,
            _ => false,
        }
    }
    fn is_circle(&self) -> bool {
        match self {
            CurveSegment::Circle { .. } => true,
            _ => false,
        }
    }
    fn radius(&self) -> f32 {
        match self {
            CurveSegment::Circle { radius, .. } => *radius,
            CurveSegment::Line { .. } => f32::INFINITY,
        }
    }
    fn angle(&self) -> f32 {
        match self {
            CurveSegment::Circle { angle, .. } => *angle,
            CurveSegment::Line { .. } => 0.0,
        }
    }
}

#[derive(Component, Clone)]
struct Curve {
    head: Vec2,
    path: Vec<CurveSegment>,
}
#[derive(Component)]
struct CurvePath;
#[derive(Component)]
struct CurveHead;

const RADIUS: f32 = 10.0;
const VEL: f32 = 20.0;
const TIMESTEP: f32 = 0.100;
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

fn collide_segments(p1: &Vec2, s1: &CurveSegment, p2: &mut Vec2, s2: &CurveSegment) -> bool {
    return false;
}

fn collide_curves(c1: &Curve, c2: &Curve) -> bool {
    if c1.path.len() == 0 || c2.path.len() == 0 {
        return false;
    }
    let s1 = c1.path.first().unwrap();
    let p1 = c1.head;
    let mut p2 = c2.head;
    for s2 in &c2.path {
        if collide_segments(&p1, s1, &mut p2, s2) {
            return true;
        }
    }
    return false;
}

fn update_collisions(mut query: Query<&Curve>) {
    for c1 in query.iter() {
        for c2 in query.iter() {
            if collide_curves(c1, c2) {
                println!("collision!");
            }
        }
    }
}

fn update_positions(mut query: Query<(&mut Position, &mut Velocity, &mut Curve, &Radius)>) {
    for (mut p, mut v, mut c, r) in query.iter_mut() {
        let dt = TIMESTEP;
        if r.0.is_infinite() {
            *p = Position(p.0 + v.0*dt);
            let head = c.head;
            if c.path.len() == 0 || !c.path.last().unwrap().is_line() {
                let new_seg = CurveSegment::Line { to: head };
                c.path.push(new_seg);
            } else {
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
            if c.path.len() == 0 || c.path.last().unwrap().radius() != r.0 || (c.path.last().unwrap().angle() - angle).abs() >= 2.0*PI {
                c.path.push(CurveSegment::Circle { 
                    center,
                    radius: r.0.abs(),
                    angle: -angle,
                });
            } else {
                let last_angle  = match c.path.last().unwrap() {
                    CurveSegment::Circle { angle, .. } => {
                        angle
                    },
                    _ => unreachable!(),
                };
                *c.path.last_mut().unwrap() = CurveSegment::Circle {
                    center,
                    radius: r.0.abs(),
                    angle: -angle + last_angle,
                };
            }
        }
        c.head = p.0;
    }
}

fn update_paths_system(
    q_curve: Query<(&Curve, &Children)>,
    mut q_path: Query<(&CurvePath, &mut Path), Without<CurveHead>,>,
    mut q_head: Query<(&CurveHead, &mut Path), Without<CurvePath>,>,
) {
    for (c, children) in q_curve.iter() {
        for child in children.iter() {
            if let Ok((_, mut p)) = q_head.get_mut(*child) {
                *p = curve_to_head(c);
            } else if let Ok((_, mut p)) = q_path.get_mut(*child) {
                *p = curve_to_path(c);
            }
        }
    }
}

fn curve_to_path(curve: &Curve) -> Path {
    let mut builder = PathBuilder::new();
    builder.move_to(curve.head);
    for s in curve.path.iter().rev() {
        match s {
            CurveSegment::Line { to } => {
                builder.line_to(*to);
            },
            CurveSegment::Circle { center, radius, angle } => {
                builder.arc(*center, Vec2::new(*radius, *radius), *angle, 0.0);
            },
        }
    }
    builder.build()
}

fn curve_to_head(curve: &Curve) -> Path {
    GeometryBuilder::build_as(&Circle {
        radius: 10.,
        center: curve.head,
    })
}

fn setup_system(mut commands: Commands) {
    let curve = Curve {
        head: Vec2::ZERO,
        path: vec![],
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Name::new("Curve 1".to_owned()),
        SpatialBundle {
            ..Default::default()
        },
        curve.clone(),
        Player,
        Position(curve.head),
        Velocity(Vec2::new(VEL, 0.0)),
        Radius(f32::INFINITY),
    )).with_children(|parent| {
        parent.spawn((
            ShapeBundle {
                path: curve_to_path(&curve),
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
            CurvePath,
        ));
        parent.spawn((
            ShapeBundle {
                path: curve_to_head(&curve),
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
            CurveHead,
        ));
    });
}
