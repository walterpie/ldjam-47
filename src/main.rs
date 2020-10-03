use std::mem;

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::physics::*;
use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::dynamics::RigidBodySet;
use rapier3d::geometry::ColliderBuilder;

use character::*;
use proc::*;
use room::*;

pub mod character;
pub mod navmesh;
pub mod proc;
pub mod room;

fn main() {
    App::build()
        .add_plugin(RapierPhysicsPlugin)
        .add_default_plugins()
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .add_system(room_system.system())
        .add_system(visible_parent_system.system())
        .add_system(character_controller_system.system())
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
    let mut dynamic = None;
    commands
        .spawn(CharBundle {
            controller: Character::default(),
            body: RigidBodyBuilder::new_kinematic().translation(0.0, 3.0, 0.0),
            collider: ColliderBuilder::cuboid(0.5, 1.0, 0.5),
        })
        .with_bundle(PbrComponents {
            mesh: char_player,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .for_current_entity(|e| character = Some(e))
        .spawn((
            CharacterDynamics,
            RigidBodyBuilder::new_dynamic().translation(0.0, 3.0, 0.0),
            ColliderBuilder::cuboid(0.5, 1.0, 0.5),
        ))
        .for_current_entity(|e| dynamic = Some(e))
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
    mut bodies: ResMut<RigidBodySet>,
    query: Query<Without<ActiveRoom, (&Edges, &Name)>>,
    connected: Query<(&RigidBodyHandleComponent, Mut<Draw>)>,
) {
    let current = current.entity;
    if let Ok(name) = query.get::<Name>(current) {
        let mut draw = connected.get_mut::<Draw>(current).unwrap();
        draw.is_visible = true;
        commands.insert_one(current, ActiveRoom);
        eprintln!("* Entering {:?}", name.get());
    }

    if let Ok(edges) = query.get::<Edges>(current) {
        for edge in edges.iter() {
            let body = connected
                .get::<RigidBodyHandleComponent>(edge.entity())
                .unwrap();
            bodies
                .get_mut(body.handle())
                .unwrap()
                .set_position(edge.isometry());
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
