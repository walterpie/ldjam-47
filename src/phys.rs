use std::mem;

use bevy::math::*;
use bevy::prelude::*;
use itertools::Itertools;

use crate::array::Array;

pub const INF_MASS: f32 = 0.0;

#[derive(Debug, Clone, Copy)]
pub struct Friction(pub f32);

impl Default for Friction {
    fn default() -> Self {
        Self(0.95)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Vec2,
    max: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    offset: Vec2,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Dynamic,
    Static,
    Semikinematic,
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub accumulator: Vec2,
    pub shape: Array<Shape, 8>,
    pub status: Status,
    pub inv_mass: f32,
    pub restitution: f32,
    pub active: bool,
    pub sensor: bool,
}

impl RigidBody {
    pub fn new(status: Status, mass: f32, restitution: f32) -> Self {
        Self {
            position: Vec2::zero(),
            rotation: 0.0,
            velocity: Vec2::zero(),
            accumulator: Vec2::zero(),
            shape: Array::new(),
            status,
            inv_mass: if mass == INF_MASS { 0.0 } else { mass.recip() },
            restitution,
            active: true,
            sensor: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_sensor(&mut self, sensor: bool) {
        self.sensor = sensor;
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn shape(mut self, offset: Vec2, width: f32, height: f32) -> Self {
        self.shape.push(Shape {
            offset,
            width,
            height,
        });
        self
    }

    pub fn aabbs(self) -> impl Iterator<Item = Aabb> + Clone {
        self.shape.into_iter().map(move |shape| Aabb {
            min: self.position + shape.offset - Vec2::new(shape.width, shape.height) * 0.5,
            max: self.position + shape.offset + Vec2::new(shape.width, shape.height) * 0.5,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Manifold {
    a: Entity,
    b: Entity,
    p_x: f32,
    p_y: f32,
    n_x: f32,
    n_y: f32,
}

pub fn collide(
    e1: Entity,
    e2: Entity,
    body1: RigidBody,
    body2: RigidBody,
) -> impl Iterator<Item = Manifold> {
    let a = body1.aabbs();
    let b = body2.aabbs();

    a.cartesian_product(b)
        .into_iter()
        .filter_map(move |(a, b)| {
            let b_pos = (b.min + b.max) * 0.5;
            let a_pos = (a.min + a.max) * 0.5;
            let d = b_pos - a_pos;

            let a_extent = (a.max.x() - a.min.x()) / 2.0;
            let b_extent = (b.max.x() - b.min.x()) / 2.0;
            let x_overlap = a_extent + b_extent - d.x().abs();

            if x_overlap > 0.0 {
                let a_extent = (a.max.y() - a.min.y()) / 2.0;
                let b_extent = (b.max.y() - b.min.y()) / 2.0;
                let y_overlap = a_extent + b_extent - d.y().abs();

                if y_overlap > 0.0 {
                    let n_x = if d.x() < 0.0 { -1.0 } else { 1.0 };
                    let n_y = if d.y() < 0.0 { -1.0 } else { 1.0 };
                    return Some(Manifold {
                        a: e1,
                        b: e2,
                        p_x: x_overlap,
                        p_y: y_overlap,
                        n_x,
                        n_y,
                    });
                }
            }

            None
        })
}

pub fn physics_system(
    mut commands: Commands,
    time: Res<Time>,
    friction: Res<Friction>,
    mut events: ResMut<Events<Manifold>>,
    mut query: Query<(Entity, Mut<RigidBody>, Mut<Transform>)>,
) {
    let mut manifolds = Vec::new();
    let bodies = query
        .iter()
        .iter()
        .filter_map(|(e, b, _)| if b.active { Some((e, *b)) } else { None })
        .collect::<Vec<_>>();
    for (i, (a, body1)) in bodies.iter().enumerate() {
        for (b, body2) in &bodies[i + 1..] {
            if body1.status != Status::Static || body2.status != Status::Static {
                manifolds.extend(collide(*a, *b, *body1, *body2));
            }
        }
    }

    let delta_time = time.delta.as_secs_f32();

    for &(e, ref body) in &bodies {
        let position = body.position + body.velocity * delta_time;
        let mut body = query.get_mut::<RigidBody>(e).unwrap();
        body.position = position;
    }

    for &(e, ref body) in &bodies {
        let velocity = body.velocity + body.accumulator * delta_time;
        let mut body = query.get_mut::<RigidBody>(e).unwrap();
        body.velocity = velocity;
        body.velocity *= friction.0;
        body.accumulator = Vec2::zero();
    }

    for manifold in manifolds {
        let a = query.get::<RigidBody>(manifold.a).unwrap();
        let b = query.get::<RigidBody>(manifold.b).unwrap();

        if a.sensor || b.sensor {
            events.send(manifold);
            continue;
        }

        {
            let rv = b.velocity.x() - a.velocity.x();

            let restitution = a.restitution.min(b.restitution);

            let mut j = -(1.0 + restitution) * rv;
            j /= a.inv_mass + b.inv_mass;

            let impulse = j * manifold.n_x;
            let percent = 0.2;
            let slop = 0.01;
            let correction =
                manifold.n_x * (manifold.p_x - slop).max(0.0) / (a.inv_mass + b.inv_mass) * percent;
            mem::drop(a);
            mem::drop(b);

            let mut a = query.get_mut::<RigidBody>(manifold.a).unwrap();
            let inv_mass = a.inv_mass;
            *a.velocity.x_mut() -= impulse * inv_mass;
            *a.position.x_mut() -= inv_mass * correction;
            mem::drop(a);
            let mut b = query.get_mut::<RigidBody>(manifold.b).unwrap();
            let inv_mass = b.inv_mass;
            *b.velocity.x_mut() += impulse * inv_mass;
            *b.position.x_mut() += inv_mass * correction;
            mem::drop(b);
        }

        let a = query.get::<RigidBody>(manifold.a).unwrap();
        let b = query.get::<RigidBody>(manifold.b).unwrap();

        {
            let rv = b.velocity.y() - a.velocity.y();

            let restitution = a.restitution.min(b.restitution);

            let mut j = -(1.0 + restitution) * rv;
            j /= a.inv_mass + b.inv_mass;

            let impulse = j * manifold.n_y;
            let percent = 0.2;
            let slop = 0.01;
            let correction =
                manifold.n_y * (manifold.p_y - slop).max(0.0) / (a.inv_mass + b.inv_mass) * percent;
            mem::drop(a);
            mem::drop(b);

            let mut a = query.get_mut::<RigidBody>(manifold.a).unwrap();
            let inv_mass = a.inv_mass;
            *a.velocity.y_mut() -= impulse * inv_mass;
            *a.position.y_mut() -= inv_mass * correction;
            mem::drop(a);
            let mut b = query.get_mut::<RigidBody>(manifold.b).unwrap();
            let inv_mass = b.inv_mass;
            *b.velocity.y_mut() += impulse * inv_mass;
            *b.position.y_mut() += inv_mass * correction;
            mem::drop(b);
        }
    }

    for &(e, ref b) in &bodies {
        let mut transform = query.get_mut::<Transform>(e).unwrap();
        transform.set_translation(Vec3::new(b.position.x(), 0.0, b.position.y()));
        transform.set_rotation(Quat::from_rotation_y(b.rotation));
    }
}
