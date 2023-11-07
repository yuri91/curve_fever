use crate::components::*;
use bevy::prelude::*;

pub trait Collidable {
    fn arc_collision(&self, a: &Arc) -> bool;
}

impl Collidable for Line {
    fn arc_collision(&self, a: &Arc) -> bool {
        match circle_to_line(a.radius, a.center, self.from, self.to) {
            (Some(p0), Some(p1)) => {
                point_in_arc(p0, &a) || point_in_arc(p1, &a)
            },
            (None, Some(p)) | (Some(p), None) => {
                point_in_arc(p, &a)
            },
            (None, None) => {
                false
            }
        }
    }
}

impl Collidable for Arc {
    fn arc_collision(&self, a: &Arc) -> bool {
        let r0 = a.radius;
        let r1 = self.radius;
        let p0 = a.center;
        let p1 = self.center;

        if let Some((p2, p3)) = circle_to_circle(r0, p0, r1, p1) {
            (point_in_arc(p2, &a) && point_in_arc(p2, self)) ||
            (point_in_arc(p3, &a) && point_in_arc(p3, self))
        } else {
            false
        }
    }
}

fn circle_to_line(r: f32, c0: Vec2, x0: Vec2, x1: Vec2) -> (Option<Vec2>, Option<Vec2>) {
    let d = x1 - x0;
    let f = x0 - c0;

    let a = d.dot(d);
    let b = 2.*f.dot(d);
    let c = f.dot(f) - r*r;
    
    let delta2 = b*b - 4.*a*c;
    if delta2 < 0. {
        return (None, None);
    }
    let delta = delta2.sqrt();
    let t0 = (-b - delta) / (2.*a);
    let t1 = (-b + delta) / (2.*a);

    (
        Some(t0).filter(|&t| t >= 0. && t <= 1.).map(|t| x0 + t*d),
        Some(t1).filter(|&t| t >= 0. && t <= 1.).map(|t| x0 + t*d),
    )
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
