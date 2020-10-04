use std::ops::Not;

use bevy::prelude::*;
use hashbrown::{HashMap, HashSet};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::phys::*;
use crate::room::*;

pub mod walls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Door {
    // -z
    North,
    // +z
    South,
    // -x
    East,
    // +x
    West,
}

impl Distribution<Door> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Door {
        match rng.gen::<u8>() % 4 {
            0 => Door::North,
            1 => Door::South,
            2 => Door::East,
            3 => Door::West,
            _ => unreachable!(),
        }
    }
}

impl Door {
    pub fn rotation(self, other: Self) -> f32 {
        use Door::East as E;
        use Door::North as N;
        use Door::South as S;
        use Door::West as W;
        let degrees: f32 = match (self, other) {
            (a, b) if a == b => 180.0,
            (a, b) if a == !b => 0.0,
            (N, E) => 90.0,
            (E, S) => 90.0,
            (S, W) => 90.0,
            (W, N) => 90.0,
            (E, N) => -90.0,
            (S, E) => -90.0,
            (W, S) => -90.0,
            (N, W) => -90.0,
            _ => unreachable!(),
        };
        degrees.to_radians()
    }

    pub fn origin(self) -> Vec3 {
        match self {
            Self::North => Vec3::new(0.0, 0.0, -1.0),
            Self::South => Vec3::new(0.0, 0.0, 1.0),
            Self::East => Vec3::new(-1.0, 0.0, 0.0),
            Self::West => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

impl Not for Door {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Door::North => Door::South,
            Door::South => Door::North,
            Door::East => Door::West,
            Door::West => Door::East,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoomPrototype {
    pub width: f32,
    pub depth: f32,
    pub height: f32,
    pub doors: HashSet<Door>,
    pub edges: Vec<EdgePrototype>,
}

#[derive(Debug, Clone)]
pub struct EdgePrototype {
    pub index: usize,
    pub from: Door,
    pub to: Door,
}

#[derive(Debug, Clone)]
pub struct LevelPrototype {
    pub start: usize,
    pub rooms: Vec<RoomPrototype>,
}

#[derive(Debug, Clone)]
pub struct DcEdge {
    index: usize,
    i: usize,
    j: usize,
    a: Entity,
    b: Entity,
}

#[derive(Debug, Clone)]
pub struct Dcg {
    edges: Vec<DcEdge>,
}

impl Dcg {
    pub fn new(edges: Vec<DcEdge>) -> Self {
        Self { edges }
    }
}

pub fn spawn(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    level: &LevelPrototype,
) {
    let prop_door = assets
        .load_sync(meshes, "assets/mesh/prop_door.gltf")
        .unwrap();
    let phys_door = assets
        .load_sync(meshes, "assets/mesh/phys_door.gltf")
        .unwrap();
    let mut edges = Vec::new();

    for (i, room) in level.rooms.iter().enumerate() {
        let handle = meshes.add(walls::generate(
            room.width,
            room.height,
            room.depth,
            &room.doors,
        ));
        let mesh = meshes.get(&handle).unwrap();
        let mut current = None;
        let mut body = RigidBody::new(Status::Static, INF_MASS, 0.5);
        body.set_active(false);
        if room.doors.contains(&Door::North) {
            let o1 = Vec2::new(-room.width / 2.0, -room.depth / 2.0 - 0.1);
            let o2 = Vec2::new(0.5, -room.depth / 2.0 - 0.1);
            body =
                body.shape(o1, room.width / 2.0 - 0.5, 0.1)
                    .shape(o2, room.width / 2.0 - 0.5, 0.1);
        } else {
            let offset = Vec2::new(-room.width / 2.0, -room.depth / 2.0 - 0.1);
            body = body.shape(offset, room.width, 0.1);
        }
        if room.doors.contains(&Door::South) {
            let o1 = Vec2::new(-room.width / 2.0, room.depth / 2.0);
            let o2 = Vec2::new(0.5, room.depth / 2.0);
            body =
                body.shape(o1, room.width / 2.0 - 0.5, 0.1)
                    .shape(o2, room.width / 2.0 - 0.5, 0.1);
        } else {
            let offset = Vec2::new(-room.width / 2.0, room.depth / 2.0);
            body = body.shape(offset, room.width, 0.1);
        }
        if room.doors.contains(&Door::East) {
            let o1 = Vec2::new(-room.width / 2.0 - 0.1, -room.depth / 2.0);
            let o2 = Vec2::new(-room.width / 2.0 - 0.1, 0.5);
            body =
                body.shape(o1, 0.1, room.depth / 2.0 - 0.5)
                    .shape(o2, 0.1, room.depth / 2.0 - 0.5);
        } else {
            let offset = Vec2::new(-room.width / 2.0 - 0.1, -room.depth / 2.0);
            body = body.shape(offset, 0.1, room.depth);
        }
        if room.doors.contains(&Door::West) {
            let o1 = Vec2::new(room.width / 2.0, -room.depth / 2.0);
            let o2 = Vec2::new(room.width / 2.0, 0.5);
            body =
                body.shape(o1, 0.1, room.depth / 2.0 - 0.5)
                    .shape(o2, 0.1, room.depth / 2.0 - 0.5);
        } else {
            let offset = Vec2::new(room.width / 2.0, -room.depth / 2.0);
            body = body.shape(offset, 0.1, room.depth);
        }

        commands
            .spawn(RoomBundle {
                name: Name::new("Unnamed".to_string()),
                body,
            })
            .with_bundle(PbrComponents {
                draw: Draw {
                    is_visible: false,
                    ..Default::default()
                },
                mesh: handle,
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                ..Default::default()
            })
            .for_current_entity(|e| {
                current = Some(e);
                edges.push((i, e, room.edges.clone()));
            });
        let current = current.unwrap();
        for &door in &room.doors {
            let translation = match door {
                Door::North | Door::South => Vec3::new(0.0, 0.0, -room.depth / 2.0),
                Door::East | Door::West => Vec3::new(0.0, 0.0, -room.width / 2.0),
            };
            let rotation = match door {
                Door::North => Quat::from_rotation_y(0.0),
                Door::South => Quat::from_rotation_y(180.0_f32.to_radians()),
                Door::East => Quat::from_rotation_y(90.0_f32.to_radians()),
                Door::West => Quat::from_rotation_y(-90.0_f32.to_radians()),
            };
            let translation = rotation * translation;
            let rotation = match door {
                Door::North => 0.0,
                Door::South => 180.0_f32.to_radians(),
                Door::East => 90.0_f32.to_radians(),
                Door::West => -90.0_f32.to_radians(),
            };
            let mesh = meshes.get(&phys_door).unwrap();
            let mut body = RigidBody::new(Status::Static, INF_MASS, 0.5)
                .shape(Vec2::zero(), 0.5, 0.1)
                .position(Vec2::new(translation.x(), translation.z()))
                .rotation(rotation);
            body.set_sensor(true);
            commands
                .spawn(PbrComponents {
                    draw: Draw {
                        is_visible: false,
                        ..Default::default()
                    },
                    mesh: prop_door,
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    ..Default::default()
                })
                .with(Parent(current))
                .with(body);
        }
    }

    let (_, current, _) = edges[level.start];
    commands.insert_resource(CurrentRoom { entity: current });

    let mut dcg = Vec::new();

    for &(i, a, ref con) in &edges {
        for (index, edge) in con.iter().enumerate() {
            let (j, b, _) = edges[edge.index];
            dcg.push(DcEdge { index, i, j, a, b });
        }
    }

    let dcg = Dcg::new(dcg);

    let mut edges: HashMap<Entity, Edges> = HashMap::new();

    for edge in dcg.edges {
        let i = &level.rooms[edge.i];
        let j = &level.rooms[edge.j];
        let prototype = &level.rooms[edge.i].edges[edge.index];

        let rotation = prototype.from.rotation(prototype.to);
        let rotation = Quat::from_rotation_y(rotation);

        let i_size = Vec3::new(i.width / 2.0, i.height / 2.0, i.depth / 2.0);
        let j_size = Vec3::new(j.width / 2.0, j.height / 2.0, j.depth / 2.0);

        let origin = match (prototype.from, prototype.to) {
            (Door::North, Door::North) => Vec3::new(0.0, 0.0, -(i_size.z() + j_size.z())),
            (Door::North, Door::South) => Vec3::new(0.0, 0.0, -(i_size.z() + j_size.z())),
            (Door::North, Door::East) => Vec3::new(0.0, 0.0, -(i_size.z() + j_size.x())),
            (Door::North, Door::West) => Vec3::new(0.0, 0.0, -(i_size.z() + j_size.x())),
            (Door::South, Door::North) => Vec3::new(0.0, 0.0, i_size.z() + j_size.z()),
            (Door::South, Door::South) => Vec3::new(0.0, 0.0, i_size.z() + j_size.z()),
            (Door::South, Door::East) => Vec3::new(0.0, 0.0, i_size.z() + j_size.x()),
            (Door::South, Door::West) => Vec3::new(0.0, 0.0, i_size.z() + j_size.x()),
            (Door::East, Door::North) => Vec3::new(-(i_size.x() + j_size.z()), 0.0, 0.0),
            (Door::East, Door::South) => Vec3::new(-(i_size.x() + j_size.z()), 0.0, 0.0),
            (Door::East, Door::East) => Vec3::new(-(i_size.x() + j_size.x()), 0.0, 0.0),
            (Door::East, Door::West) => Vec3::new(-(i_size.x() + j_size.x()), 0.0, 0.0),
            (Door::West, Door::North) => Vec3::new(i_size.x() + j_size.z(), 0.0, 0.0),
            (Door::West, Door::South) => Vec3::new(i_size.x() + j_size.z(), 0.0, 0.0),
            (Door::West, Door::East) => Vec3::new(i_size.x() + j_size.x(), 0.0, 0.0),
            (Door::West, Door::West) => Vec3::new(i_size.x() + j_size.x(), 0.0, 0.0),
        };

        edges
            .entry(edge.a)
            .or_default()
            .add_mut(Room::new(edge.b).origin(origin).rotation(rotation));
    }

    for (entity, edge) in edges {
        commands.insert_one(entity, edge);
    }
}

#[derive(Default)]
pub struct Parameters {
    pub size: usize,
    pub min_size: f32,
    pub max_size: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub clone_probability: f32,
}

pub fn generate(params: &Parameters) -> LevelPrototype {
    let mut size = params.size;

    let start = 0;
    let mut rooms = Vec::new();

    while size > 0 {
        let width = rand::random::<f32>() * (params.max_size - params.min_size) + params.min_size;
        let depth = rand::random::<f32>() * (params.max_size - params.min_size) + params.min_size;
        let height =
            rand::random::<f32>() * (params.max_height - params.min_height) + params.min_height;
        let room = RoomPrototype {
            width,
            height,
            depth,
            doors: HashSet::new(),
            edges: Vec::new(),
        };
        if rand::random::<f32>() < params.clone_probability {
            rooms.push(room.clone());
            size -= 1;
        }
        if size > 0 {
            rooms.push(room);
            size -= 1;
        }
    }

    let len = rooms.len();

    for (i, room) in rooms.iter_mut().enumerate() {
        let n = 1 + rand::random::<usize>() % 3;
        while room.doors.len() < n {
            room.doors.insert(rand::random());
        }
        let mut edges = Vec::new();
        for &from in &room.doors {
            let mut index;
            loop {
                index = rand::random::<usize>() % len;
                if index != i {
                    break;
                }
            }
            let to = rand::random::<Door>();
            edges.push(EdgePrototype { index, from, to });
        }
        room.edges = edges;
    }

    LevelPrototype { start, rooms }
}
