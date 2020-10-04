#![allow(incomplete_features)]
#![feature(const_generics)]

use std::mem;

use bevy::math::*;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::render_graph::base::camera::CAMERA3D;
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
        .init_resource::<SensorListenerState>()
        .add_event::<Manifold>()
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::LAST, room_system.system())
        .add_system_to_stage(stage::LAST, visible_parent_system.system())
        .add_system_to_stage(stage::FIRST, character_controller_system.system())
        .add_system_to_stage(stage::POST_UPDATE, sensor_system.system())
        .add_system_to_stage(stage::UPDATE, physics_system.system())
        .add_system_to_stage(stage::UPDATE, joints_system.system())
        .add_system_to_stage(stage::UPDATE, debug_draw_system.system())
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
    let mut sensor = None;
    let mut sensor_body =
        RigidBody::new(Status::Semikinematic, 1.0, 0.5).shape(Vec2::new(-2.0, -2.0), 4.0, 4.0);
    sensor_body.set_sensor(true);
    commands
        .spawn(CharBundle {
            global_transform: Default::default(),
            transform: Default::default(),
            controller: Character::default(),
            body: RigidBody::new(Status::Semikinematic, 1.0, 0.5).shape(
                Vec2::new(-0.25, -0.25),
                0.5,
                0.5,
            ),
        })
        .for_current_entity(|e| character = Some(e))
        .spawn(SensorBundle {
            global_transform: Default::default(),
            transform: Default::default(),
            controller: Sensor {
                character: character.unwrap(),
            },
            body: sensor_body,
        })
        .for_current_entity(|e| sensor = Some(e))
        .spawn((Joint::new(character.unwrap(), sensor.unwrap()),))
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 1.6, 0.0)),
            ..Default::default()
        })
        .with(Parent(character.unwrap()))
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 1.6, 0.0)),
            camera: Camera {
                name: Some(CAMERA3D.to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(FirstPersonCamera)
        .with(Parent(character.unwrap()))
        .spawn(Camera3dComponents {
            camera: Camera {
                name: Some("None".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(FlyCamera::default());
    let params = Parameters {
        size: 10,
        min_size: 4.0,
        max_size: 16.0,
        min_height: 2.0,
        max_height: 2.0,
        clone_probability: 1.0,
    };
    let level = proc::generate(&params);
    // let level = LevelPrototype {
    //     start: 0,
    //     rooms: vec![
    //         RoomPrototype {
    //             width: 8.0,
    //             height: 2.0,
    //             depth: 8.0,
    //             doors: vec![Door::South].into_iter().collect(),
    //             edges: vec![EdgePrototype {
    //                 index: 1,
    //                 from: Door::South,
    //                 to: Door::North,
    //             }],
    //         },
    //         RoomPrototype {
    //             width: 8.0,
    //             height: 2.0,
    //             depth: 8.0,
    //             doors: vec![].into_iter().collect(),
    //             edges: vec![],
    //         },
    //     ],
    // };
    proc::spawn(&mut commands, &assets, &mut meshes, &mut materials, &level);
}

pub fn room_system(
    mut commands: Commands,
    current: Res<CurrentRoom>,
    mut query: Query<(Entity, &Edges, &Name)>,
    mut is_active: Query<&ActiveRoom>,
    connected: Query<(Mut<RigidBody>, Mut<Draw>)>,
    mut rooms: Query<With<RoomMarker, Entity>>,
    mut connections: Query<(Entity, Mut<Connection>)>,
) {
    let any_active = is_active.iter().iter().count() != 0;
    if !any_active {
        for (e, mut connection) in &mut connections.iter() {
            let mut body = connected.get_mut::<RigidBody>(e).unwrap();
            body.set_active(false);
            if connection.open {
                body.rotation -= 90.0_f32.to_radians();
                let rot = Mat2::from_angle(body.rotation);
                let offset = rot * Vec2::new(0.5, 0.5);
                body.position -= offset;
                body.set_sensor(false);
                connection.open = false;
            }
        }
        for e in &mut rooms.iter() {
            connected.get_mut::<RigidBody>(e).unwrap().set_active(false);
            connected.get_mut::<Draw>(e).unwrap().is_visible = false;
        }
    }
    let current = current.entity;
    if let Ok(name) = query.get::<Name>(current) {
        let mut draw = connected.get_mut::<Draw>(current).unwrap();
        draw.is_visible = true;
        let mut body = connected.get_mut::<RigidBody>(current).unwrap();
        body.set_active(true);
        if is_active.get::<ActiveRoom>(current).is_err() {
            body.position = Vec2::zero();
            body.rotation = 0.0;
            commands.insert_one(current, ActiveRoom);
            eprintln!("* Entering {:?}", name.get());
        }
    }

    if let Ok(edges) = query.get::<Edges>(current) {
        for edge in edges.iter() {
            let position = Vec2::new(edge.isometry().0.x(), edge.isometry().0.z());
            // NOTE: assumes quat is (0, 1, 0, theta)
            let rotation = edge.isometry().1.to_axis_angle().1;
            let mut body = connected.get_mut::<RigidBody>(edge.entity()).unwrap();
            body.set_active(false);
            body.position = position;
            body.rotation = rotation;
            let mut draw = connected.get_mut::<Draw>(edge.entity()).unwrap();
            draw.is_visible = true;
        }
    }

    if let Ok(doorset) = query.get::<DoorSet>(current) {
        for &e in &doorset.vec {
            let mut body = connected.get_mut::<RigidBody>(e).unwrap();
            body.set_active(true);
        }
    }
}

pub fn visible_parent_system(
    mut draw_parents: Query<With<Draw, (Entity, &Parent)>>,
    mut body_parents: Query<With<RigidBody, (Entity, &Parent)>>,
    drawables: Query<Mut<Draw>>,
    bodies: Query<Mut<RigidBody>>,
    mut connections: Query<&Connection>,
) {
    for (e, parent) in &mut draw_parents.iter() {
        if let Ok(parent) = drawables.get::<Draw>(**parent) {
            let is_visible = parent.is_visible;
            mem::drop(parent);
            drawables.get_mut::<Draw>(e).unwrap().is_visible = is_visible;
        }
    }
    for conn in &mut connections.iter() {
        if let Ok(mut body) = bodies.get_mut::<RigidBody>(conn.sensor) {
            body.set_active(conn.open);
        }
    }
}
