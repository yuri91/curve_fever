use crate::components::*;
use bevy::prelude::*;

pub fn line_to_line(l0: &Line, l1: &Line) -> bool {
    false
}

pub fn arc_to_line(a: &Arc, l: &Line) -> bool {
    false
}

pub fn arc_to_arc(a0: &Arc, a1: &Arc) -> bool {
    let r0 = a0.radius;
    let r1 = a1.radius;
    let p0 = a0.center;
    let p1 = a1.center;

    if let Some((p2, p3)) = circle_to_circle(r0, p0, r1, p1) {
        (point_in_arc(p2, &a0) && point_in_arc(p2, &a1)) ||
        (point_in_arc(p3, &a0) && point_in_arc(p3, &a1))
    } else {
        false
    }
}

fn point_in_arc(p: Vec2, a: &Arc) -> bool {
    let c = a.center;
    let x0 = a.from;
    let v0 = x0 - c;
    let v1 = p - c;
    let alpha = v0.angle_between(v1);
    return alpha.abs() < a.angle.abs() && alpha.signum() == a.angle.signum();
}

fn circle_to_circle(r0: f32, p0: Vec2, r1: f32, p1: Vec2) -> Option<(Vec2, Vec2)> {
    let d = p1.distance(p0);
    if d > r0 + r1 {
        return None;
    }
    if d < (r0 - r1).abs() {
        return None;
    }
    let a = (r0*r0 - r1*r1 + d*d) / (2.*d);
    let h = (r0*r0 - a*a).sqrt();
    let x2 = p0.x + a*(p1.x-p0.x)/d;
    let y2 = p0.y + a*(p1.y-p0.y)/d;
    let x3 = x2 + h*(p1.y-p0.y)/d;
    let y3 = y2 - h*(p1.x-p0.x)/d;
    let x4 = x2 - h*(p1.y-p0.y)/d;
    let y4 = y2 + h*(p1.x-p0.x)/d;
    Some((Vec2::new(x3, y3), Vec2::new(x4, y4)))
}
