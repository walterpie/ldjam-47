#![allow(incomplete_features)]
#![feature(const_generics)]

use std::mem;

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

use character::*;
use phys::*;
use proc::*;
use room::*;

pub mod array;
pub mod character;
pub mod phys;
pub mod proc;
pub mod room;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(FlyCameraPlugin)
        .init_resource::<Friction>()
        .add_startup_system(setup.system())
        .add_system(room_system.system())
        .add_system(visible_parent_system.system())
        .add_system(character_controller_system.system())
        .add_system(physics_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let char_player = assets
        .load_sync(&mut meshes, "assets/mesh/char_player.gltf")
        .unwrap();
    let mut character = None;
    commands
        .spawn(CharBundle {
            controller: Character::default(),
            body: RigidBody::new(Status::Semikinematic, 1.0, 0.5)
                .position(Vec2::new(0.0, 0.0))
                .shape(Vec2::zero(), 0.5, 0.5),
        })
        .with_bundle(PbrComponents {
            mesh: char_player,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .for_current_entity(|e| character = Some(e))
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation_rotation(
                Vec3::new(0.0, 8.0, 7.0),
                Quat::from_rotation_x(-30.0_f32.to_radians()),
            ),
            ..Default::default()
        })
        .with(Parent(character.unwrap()));
    let params = Parameters {
        size: 10,
        min_size: 4.0,
        max_size: 16.0,
        min_height: 2.0,
        max_height: 4.0,
        clone_probability: 0.3,
    };
    let level = proc::generate(&params);
    proc::spawn(&mut commands, &assets, &mut meshes, &mut materials, &level);
}

pub fn room_system(
    mut commands: Commands,
    current: Res<CurrentRoom>,
    query: Query<Without<ActiveRoom, (&Edges, &Name)>>,
    connected: Query<(Mut<RigidBody>, Mut<Draw>)>,
) {
    let current = current.entity;
    if let Ok(name) = query.get::<Name>(current) {
        let mut draw = connected.get_mut::<Draw>(current).unwrap();
        draw.is_visible = true;
        connected
            .get_mut::<RigidBody>(current)
            .unwrap()
            .set_active(true);
        commands.insert_one(current, ActiveRoom);
        eprintln!("* Entering {:?}", name.get());
    }

    if let Ok(edges) = query.get::<Edges>(current) {
        for edge in edges.iter() {
            let position = Vec2::new(edge.isometry().0.x(), edge.isometry().0.z());
            // NOTE: assumes quat is (0, 1, 0, theta)
            let rotation = edge.isometry().1.to_axis_angle().1;
            let mut body = connected.get_mut::<RigidBody>(edge.entity()).unwrap();
            body.set_active(true);
            body.position = position;
            body.rotation = rotation;
            let mut draw = connected.get_mut::<Draw>(edge.entity()).unwrap();
            draw.is_visible = true;
        }
    }
}

pub fn visible_parent_system(
    mut parents: Query<With<Draw, (Entity, &Parent)>>,
    drawables: Query<Mut<Draw>>,
) {
    for (e, parent) in &mut parents.iter() {
        if let Ok(parent) = drawables.get::<Draw>(**parent) {
            let is_visible = parent.is_visible;
            mem::drop(parent);
            drawables.get_mut::<Draw>(e).unwrap().is_visible = is_visible;
        }
    }
}
