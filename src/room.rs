use bevy::prelude::*;
use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::geometry::ColliderBuilder;
use rapier3d::math::*;
use rapier3d::na::{Translation3, UnitQuaternion};

#[derive(Debug)]
pub struct CurrentRoom {
    pub entity: Entity,
}

#[derive(Default, Debug)]
pub struct ActiveRoom;

#[derive(Bundle)]
pub struct RoomBundle {
    pub name: Name,
    pub body: RigidBodyBuilder,
    pub collider: ColliderBuilder,
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
pub struct Room {
    entity: Entity,
    isometry: Isometry<f32>,
}

impl Room {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            isometry: Isometry::identity(),
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn isometry(&self) -> Isometry<f32> {
        self.isometry
    }

    pub fn origin(mut self, origin: Translation3<f32>) -> Self {
        self.isometry.translation = origin;
        self
    }

    pub fn rotation(mut self, rotation: UnitQuaternion<f32>) -> Self {
        self.isometry.rotation = rotation;
        self
    }
}
