use bevy::prelude::*;

use crate::phys::*;

#[derive(Debug)]
pub struct CurrentRoom {
    pub entity: Entity,
}

#[derive(Default, Debug)]
pub struct ActiveRoom;

pub struct RoomMarker;

#[derive(Default, Debug)]
pub struct DoorSet {
    pub vec: Vec<Entity>,
}

#[derive(Bundle)]
pub struct RoomBundle {
    pub marker: RoomMarker,
    pub name: Name,
    pub body: RigidBody,
    pub props: Props,
}

#[derive(Debug)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get(&self) -> &str {
        &self.name
    }
}

#[derive(Default, Debug)]
pub struct Edges {
    edges: Vec<Room>,
}

impl Edges {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn add(mut self, edge: Room) -> Self {
        self.edges.push(edge);
        self
    }

    pub fn add_mut(&mut self, edge: Room) {
        self.edges.push(edge);
    }

    pub fn get(&self, i: usize) -> Option<&Room> {
        self.edges.get(i)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Room> {
        self.edges.iter()
    }
}

#[derive(Debug)]
pub struct Props {
    pub vec: Vec<Entity>,
}

#[derive(Debug)]
pub struct Room {
    entity: Entity,
    position: Vec3,
    rotation: Quat,
}

impl Room {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            position: Vec3::zero(),
            rotation: Quat::identity(),
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn isometry(&self) -> (Vec3, Quat) {
        (self.position, self.rotation)
    }

    pub fn origin(mut self, origin: Vec3) -> Self {
        self.position = origin;
        self
    }

    pub fn rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }
}
