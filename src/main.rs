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
pub mod faux;
pub mod level;
pub mod phys;
pub mod proc;
pub mod room;
pub mod text;

pub const FAUX: bool = false;

fn main() {
    let mut builder = App::build();
    builder
        .add_default_plugins()
        .add_plugin(FlyCameraPlugin)
        .init_resource::<Friction>()
        .init_resource::<SensorListenerState>()
        .add_event::<Manifold>()
        .add_startup_system(setup.system());
    if FAUX {
        builder
            .add_system_to_stage(stage::FIRST, character_controller_system.system())
            .add_system_to_stage(stage::UPDATE, physics_system.system())
            .add_system_to_stage(stage::UPDATE, joints_system.system())
            .run()
    } else {
        builder
            .add_system_to_stage(stage::LAST, room_system.system())
            .add_system_to_stage(stage::LAST, visible_parent_system.system())
            .add_system_to_stage(stage::FIRST, character_controller_system.system())
            .add_system_to_stage(stage::POST_UPDATE, sensor_system.system())
            .add_system_to_stage(stage::UPDATE, physics_system.system())
            .add_system_to_stage(stage::UPDATE, joints_system.system())
            .add_system_to_stage(stage::UPDATE, debug_draw_system.system())
            .add_system_to_stage(stage::UPDATE, text::text_system.system())
            .run()
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fonts: ResMut<Assets<Font>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ui_mats: ResMut<Assets<ColorMaterial>>,
) {
    let char_player = assets
        .load_sync(&mut meshes, "assets/mesh/char_player.gltf")
        .unwrap();
    let font = assets
        .load_sync(&mut fonts, "assets/font/TruenoLight-E2pg.ttf")
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
        .with(FlyCamera::default())
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: ui_mats.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    style: Style {
                        // align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(50.0), Val::Px(80.0)),
                        ..Default::default()
                    },
                    material: ui_mats.add(Color::rgba(0.05, 0.05, 0.05, 0.8).into()),
                    ..Default::default()
                })
                .with(text::TextFrame(font, false))
                .with_children(|_| {})
                .spawn(NodeComponents {
                    style: Style {
                        // align_self: AlignSelf::Center,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Auto, Val::Auto),
                        min_size: Size::new(Val::Auto, Val::Px(40.0)),
                        max_size: Size::new(Val::Percent(100.0), Val::Px(120.0)),
                        ..Default::default()
                    },
                    material: ui_mats.add(Color::rgba(0.05, 0.05, 0.05, 0.8).into()),
                    ..Default::default()
                })
                .with(text::TextFrame(font, true))
                .with_children(|_| {});
        });
    // let params = Parameters {
    //     size: 10,
    //     min_size: 4.0,
    //     max_size: 16.0,
    //     min_height: 2.0,
    //     max_height: 2.0,
    //     min_props: 0,
    //     max_props: 3,
    //     props: ["bed", "chair", "desk", "flower_table"]
    //         .iter()
    //         .map(|n| n.to_string())
    //         .collect(),
    //     clone_probability: 1.0,
    // };
    // let level = proc::generate(&params);
    if FAUX {
        faux::spawn(&mut commands, &assets, &mut meshes, &mut materials);
    } else {
        let level = level::new();
        proc::spawn(&mut commands, &assets, &mut meshes, &mut materials, &level);
    }
}

pub fn room_system(
    mut commands: Commands,
    current: Res<CurrentRoom>,
    mut query: Query<(Entity, &Edges, &Name, &Props)>,
    mut is_active: Query<&ActiveRoom>,
    connected: Query<(Mut<RigidBody>, Mut<Draw>)>,
    mut rooms: Query<With<RoomMarker, Entity>>,
    mut connections: Query<(Entity, Mut<Connection>)>,
    mut frames: Query<(Entity, &text::TextFrame, &Children)>,
) {
    let any_active = is_active.iter().iter().count() != 0;
    if !any_active {
        for (e, mut connection) in &mut connections.iter() {
            let mut body = connected.get_mut::<RigidBody>(e).unwrap();
            body.set_active(false);
            if connection.open {
                body.rotation -= 90.0_f32.to_radians();
                let rot = Mat2::from_angle(-body.rotation);
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
            for (e, &text::TextFrame(font, is_desc), children) in &mut frames.iter() {
                for &child in children.iter() {
                    commands.despawn_recursive(child);
                }
                if !is_desc {
                    commands
                        .spawn(TextComponents {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(80.0)),
                                ..Default::default()
                            },
                            text: Text {
                                value: name.get().to_string(),
                                font,
                                style: TextStyle {
                                    font_size: 80.0,
                                    color: Color::WHITE,
                                },
                            },
                            ..Default::default()
                        })
                        .with(text::Fading {
                            alpha: 3.0,
                            fade: 0.5,
                        })
                        .with(Parent(e));
                } else {
                    for (i, line) in name.description().lines().enumerate() {
                        commands
                            .spawn(TextComponents {
                                style: Style {
                                    align_self: AlignSelf::FlexStart,
                                    position: Rect {
                                        top: Val::Px(40.0 * i as f32),
                                        bottom: Val::Auto,
                                        ..Default::default()
                                    },
                                    size: Size::new(Val::Auto, Val::Px(40.0)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: format!("* {} *", line),
                                    font,
                                    style: TextStyle {
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                    },
                                },
                                ..Default::default()
                            })
                            .with(text::Fading {
                                alpha: 2.0,
                                fade: 0.5,
                            })
                            .with(Parent(e));
                    }
                }
            }
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

    if let Ok(props) = query.get::<Props>(current) {
        for &e in &props.vec {
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
