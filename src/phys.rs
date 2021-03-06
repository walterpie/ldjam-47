use std::mem;

use bevy::math::*;
use bevy::prelude::*;
use bevy::render::{mesh::*, pipeline::PrimitiveTopology, prelude::*};
use hashbrown::HashSet;
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
pub struct Joint {
    body1: Entity,
    body2: Entity,
    offset: Vec2,
    angle: f32,
}

impl Joint {
    pub fn new(body1: Entity, body2: Entity) -> Self {
        Self {
            body1,
            body2,
            offset: Vec2::zero(),
            angle: 0.0,
        }
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }
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

    pub fn is_active(&self) -> bool {
        self.active
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
        self.shape.into_iter().map(move |shape| {
            let mut min = shape.offset;
            let mut max = shape.offset + Vec2::new(shape.width, shape.height);
            let rotation = Mat2::from_angle(self.rotation);
            min = self.position + rotation * min;
            max = self.position + rotation * max;
            let min_x = min.x().min(max.x());
            let min_y = min.y().min(max.y());
            let max_x = min.x().max(max.x());
            let max_y = min.y().max(max.y());
            min = Vec2::new(min_x, min_y);
            max = Vec2::new(max_x, max_y);
            Aabb { min, max }
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Manifold {
    pub a: Entity,
    pub b: Entity,
    pub p_x: f32,
    pub p_y: f32,
    pub n_x: f32,
    pub n_y: f32,
    pub a_c: [bool; 2],
    pub b_c: [bool; 2],
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
                let a_x_contained = a.min.x() > b.min.x() && a.max.x() < b.max.x();
                let b_x_contained = b.min.x() > a.min.x() && b.max.x() < a.max.x();

                let a_extent = (a.max.y() - a.min.y()) / 2.0;
                let b_extent = (b.max.y() - b.min.y()) / 2.0;
                let y_overlap = a_extent + b_extent - d.y().abs();

                if y_overlap > 0.0 {
                    let a_y_contained = a.min.y() > b.min.y() && a.max.y() < b.max.y();
                    let b_y_contained = b.min.y() > a.min.y() && b.max.y() < a.max.y();

                    let n_x = if d.x() < 0.0 { -1.0 } else { 1.0 };
                    let n_y = if d.y() < 0.0 { -1.0 } else { 1.0 };
                    return Some(Manifold {
                        a: e1,
                        b: e2,
                        p_x: x_overlap,
                        p_y: y_overlap,
                        n_x,
                        n_y,
                        a_c: [a_x_contained, a_y_contained],
                        b_c: [b_x_contained, b_y_contained],
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
    let mut bodies = query
        .iter()
        .iter()
        .map(|(e, b, _)| (e, *b))
        .collect::<Vec<_>>();

    let delta_time = time.delta.as_secs_f32();

    for &mut (e, ref mut body) in &mut bodies {
        if !body.active {
            continue;
        }
        let position = body.position + body.velocity * delta_time;
        let mut b = query.get_mut::<RigidBody>(e).unwrap();
        b.position = position;
        body.position = position;
    }

    for (i, (a, body1)) in bodies.iter().enumerate() {
        if !body1.active {
            continue;
        }
        for (b, body2) in &bodies[i + 1..] {
            if !body2.active {
                continue;
            }
            if body1.status != Status::Static || body2.status != Status::Static {
                manifolds.extend(collide(*a, *b, *body1, *body2));
            }
        }
    }

    let mut skx = HashSet::new();
    let mut sky = HashSet::new();

    let mut count = 0;
    for manifold in manifolds {
        let a = query.get::<RigidBody>(manifold.a).unwrap();
        let b = query.get::<RigidBody>(manifold.b).unwrap();

        if a.sensor || b.sensor {
            count += 1;
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
            match a.status {
                Status::Static => {}
                Status::Dynamic => {
                    let inv_mass = a.inv_mass;
                    *a.velocity.x_mut() -= impulse * inv_mass;
                    *a.position.x_mut() -= inv_mass * correction;
                }
                Status::Semikinematic => {
                    if !skx.contains(&manifold.a) {
                        skx.insert(manifold.a);
                        if !manifold.a_c[0] {
                            let d = -manifold.n_x * manifold.p_x;
                            let v = a.velocity.x() * delta_time;
                            if v.signum() != d.signum() && d.abs() < v.abs() {
                                *a.position.x_mut() += d;
                            } else {
                                *a.position.x_mut() -= v;
                            }
                        }
                    }
                }
            }
            for &mut (e, ref mut body) in &mut bodies {
                if e == manifold.a {
                    body.position = a.position;
                    body.velocity = a.velocity;
                }
            }
            mem::drop(a);
            let mut b = query.get_mut::<RigidBody>(manifold.b).unwrap();
            match b.status {
                Status::Static => {}
                Status::Dynamic => {
                    let inv_mass = b.inv_mass;
                    *b.velocity.x_mut() -= impulse * inv_mass;
                    *b.position.x_mut() -= inv_mass * correction;
                }
                Status::Semikinematic => {
                    if !skx.contains(&manifold.b) {
                        skx.insert(manifold.b);
                        if !manifold.b_c[0] {
                            let d = manifold.n_x * manifold.p_x;
                            let v = b.velocity.x() * delta_time;
                            if v.signum() != d.signum() && d.abs() < v.abs() {
                                *b.position.x_mut() += d;
                            } else {
                                *b.position.x_mut() -= v;
                            }
                        }
                    }
                }
            }
            for &mut (e, ref mut body) in &mut bodies {
                if e == manifold.b {
                    body.position = b.position;
                    body.velocity = b.velocity;
                }
            }
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
            match a.status {
                Status::Static => {}
                Status::Dynamic => {
                    let inv_mass = a.inv_mass;
                    *a.velocity.y_mut() -= impulse * inv_mass;
                    *a.position.y_mut() -= inv_mass * correction;
                }
                Status::Semikinematic => {
                    if !sky.contains(&manifold.a) {
                        if !manifold.a_c[1] {
                            sky.insert(manifold.a);
                            let d = -manifold.n_y * manifold.p_y;
                            let v = a.velocity.y() * delta_time;
                            if v.signum() != d.signum() && d.abs() < v.abs() {
                                *a.position.y_mut() += d;
                            } else {
                                *a.position.y_mut() -= v;
                            }
                        }
                    }
                }
            }
            for &mut (e, ref mut body) in &mut bodies {
                if e == manifold.a {
                    body.position = a.position;
                    body.velocity = a.velocity;
                }
            }
            mem::drop(a);
            let mut b = query.get_mut::<RigidBody>(manifold.b).unwrap();
            match b.status {
                Status::Static => {}
                Status::Dynamic => {
                    let inv_mass = b.inv_mass;
                    *b.velocity.y_mut() -= impulse * inv_mass;
                    *b.position.y_mut() -= inv_mass * correction;
                }
                Status::Semikinematic => {
                    if !sky.contains(&manifold.b) {
                        if !manifold.b_c[1] {
                            sky.insert(manifold.b);
                            let d = manifold.n_y * manifold.p_y;
                            let v = b.velocity.y() * delta_time;
                            if v.signum() != d.signum() && d.abs() < v.abs() {
                                *b.position.y_mut() += d;
                            } else {
                                *b.position.y_mut() -= v;
                            }
                        }
                    }
                }
            }
            for &mut (e, ref mut body) in &mut bodies {
                if e == manifold.b {
                    body.position = b.position;
                    body.velocity = b.velocity;
                }
            }
            mem::drop(b);
        }
    }

    for &(e, ref body) in &bodies {
        if !body.active {
            continue;
        }
        let velocity = body.velocity + body.accumulator * delta_time;
        let mut body = query.get_mut::<RigidBody>(e).unwrap();
        body.velocity = velocity;
        body.velocity *= friction.0;
        body.accumulator = Vec2::zero();
    }

    for &(e, ref b) in &bodies {
        let mut transform = query.get_mut::<Transform>(e).unwrap();
        transform.set_translation(Vec3::new(b.position.x(), 0.0, b.position.y()));
        transform.set_rotation(Quat::from_rotation_y(b.rotation));
    }
}

pub fn joints_system(query: Query<Mut<RigidBody>>, mut joints: Query<&Joint>) {
    for &Joint {
        body1,
        body2,
        offset,
        angle,
    } in &mut joints.iter()
    {
        let (position, rotation) = {
            let body = query.get::<RigidBody>(body1).unwrap();
            let position = body.position;
            let rotation = body.rotation;
            (position, rotation)
        };
        let mut body = query.get_mut::<RigidBody>(body2).unwrap();
        body.position = position + offset;
        body.rotation = rotation + angle;
    }
}

pub struct DebugDraw;

pub fn debug_draw_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<Without<DebugDraw, (Entity, &RigidBody)>>,
) {
    for (e, body) in &mut query.iter() {
        if !body.active {
            continue;
        }
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        for shape in body.shape.iter() {
            let min = shape.offset;
            let max = shape.offset + Vec2::new(shape.width, shape.height);
            let v0 = Vec3::new(min.x(), 4.0, min.y());
            let v1 = Vec3::new(min.x(), 4.0, max.y());
            let v2 = Vec3::new(max.x(), 4.0, max.y());
            let v3 = Vec3::new(max.x(), 4.0, min.y());
            let p = &[v0.into(), v1.into(), v2.into(), v3.into()];
            let n = &[
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ];
            let u = &[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
            let c = positions.len() as u32;
            positions.extend(p);
            normals.extend(n);
            uvs.extend(u);
            indices.extend(&[c + 0, c + 1]);
            indices.extend(&[c + 1, c + 2]);
            indices.extend(&[c + 2, c + 3]);
            indices.extend(&[c + 3, c + 0]);
        }
        let attributes = vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ];
        let mesh = Mesh {
            primitive_topology: PrimitiveTopology::LineList,
            attributes,
            indices: Some(indices),
        };
        let handle = meshes.add(mesh);
        let color = if body.sensor {
            Color::rgba(0.0, 0.0, 1.0, 0.0)
        } else {
            Color::rgba(1.0, 0.0, 0.0, 0.0)
        };
        commands
            .spawn(PbrComponents {
                mesh: handle,
                material: materials.add(color.into()),
                ..Default::default()
            })
            .with(Parent(e))
            .insert_one(e, DebugDraw);
    }
}
