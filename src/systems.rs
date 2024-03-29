use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

use crate::components::*;
use crate::collisions::Collidable;

const RADIUS: f32 = 10.0;
const VEL: f32 = 20.0;
const EPSILON: f32 = 0.001;

#[derive(Resource)]
pub struct NextZIndex(pub u32);

pub fn update_acceleration(keys: Res<Input<KeyCode>>, mut query: Query<&mut Radius, With<Player>>) {
    let mut r  = query.get_single_mut().unwrap();
    if keys.pressed(KeyCode::Left) {
        *r = Radius(RADIUS);
    } else if keys.pressed(KeyCode::Right) {
        *r = Radius(-RADIUS);
    } else {
        *r = Radius(f32::INFINITY);
    }
}

pub fn update_collisions<C: Collidable+Component>(
    q_heads: Query<(&Head, &Position, &Velocity )>,
    q_collidable: Query<(Entity, &mut C)>,
    mut commands: Commands,
) {
    for (h, p, v) in q_heads.iter() {
        let v_dir = v.0.normalize()*h.radius;
        let p_edge = Vec2::from_angle(-PI/2.0).rotate(v_dir) + p.0;
        let h_arc = Arc {
            center: p.0,
            from: p_edge,
            radius: h.radius,
            angle: PI,
            color: h.color,
        };
        for (e, c) in q_collidable.iter() {
            if c.arc_collision(&h_arc) {
                commands.entity(e).insert(Collided);
            }
        }
    }
}

pub fn update_positions(
    time: Res<Time>,
    mut next_z: ResMut<NextZIndex>,
    mut query: Query<(&mut Position, &mut Velocity, &mut Head, &Radius, )>,
    mut q_lines: Query<&mut Line>,
    mut q_arcs: Query<&mut Arc>,
    mut commands: Commands,
) {
    for (mut p, mut v, mut h, r,) in query.iter_mut() {
        let dt = time.delta_seconds();
        let prev_pos = p.clone().0;
        if r.0.is_infinite() {
            *p = Position(p.0 + v.0*dt);
            if let Some(mut line) = h.tail.and_then(|t| q_lines.get_mut(t).ok()) {
                line.to = p.0;
            } else {
                next_z.as_mut().0 += 1;
                h.tail = Some(commands.spawn((
                    Line {
                        from: prev_pos,
                        to: p.0,
                        color: h.color,
                    },
                    ZIdx(next_z.0 as f32),
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
                next_z.as_mut().0 += 1;
                h.tail = Some(commands.spawn((
                    Arc {
                        from: prev_pos,
                        center,
                        radius: r.0.abs(),
                        angle,
                        color: h.color,
                    },
                    ZIdx(next_z.0 as f32),
                )).id());
            }
        }
    }
}

pub fn update_paths<P: ToPath+Component>(
    query: Query<(Entity, &P, &ZIdx, Option<&Collided>), Or<(Changed<P>, Added<Collided>)>>,
    mut commands: Commands,
) {
    for (e, p, z, c) in query.iter() {
        let color = if c.is_some() {
            Color::RED
        } else {
            Color::BLACK
        };
        commands.entity(e).insert((
            ShapeBundle {
                path: p.to_path(),
                transform: Transform {
                    translation: Vec3::new(0., 0., z.0 as f32),
                    ..default()
                },
                ..default()
            },
            Stroke::new(color, 3.0),
        ));
    }
}
pub fn update_translation(
    mut query: Query<(&mut Transform, &Position, &ZIdx), Changed<Position>>,
) {
    for (mut t, p, z, ) in query.iter_mut() {
        t.translation = p.0.extend(z.0);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::new_with_far(f32::MAX));
    commands.spawn((
        Name::new("Curve 1".to_owned()),
        Player,
        Position(Vec2::ZERO),
        Velocity(Vec2::new(VEL, 0.0)),
        Radius(f32::INFINITY),
        Head { radius: 5., color: Color::BLACK, tail: None },
        ZIdx(f32::MAX-1.),
    ));
}
