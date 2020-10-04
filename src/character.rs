use std::mem;

use bevy::input::mouse::MouseMotion;
use bevy::math::*;
use bevy::prelude::*;
use bevy::render::render_graph::base::camera::CAMERA3D;
use bevy::render::{camera::*, prelude::*};
use bevy_fly_camera::FlyCamera;

use crate::phys::*;
use crate::proc::{Connection, RoomSensor};
use crate::room::*;

pub const MOUSE_SPEED: f32 = 0.03;
pub const MAX_SPEED: f32 = 2.5;
pub const INC_SPEED: f32 = 5.0;

#[derive(Default)]
pub struct FirstPersonCamera;

pub struct Character {
    active: bool,
    yrot: f32,
    xrot: f32,
    reader: EventReader<MouseMotion>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            active: true,
            yrot: 0.0,
            xrot: 0.0,
            reader: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Sensor;

#[derive(Bundle)]
pub struct CharBundle {
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub controller: Character,
    pub body: RigidBody,
}

#[derive(Bundle)]
pub struct SensorBundle {
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub controller: Sensor,
    pub body: RigidBody,
}

#[derive(Default)]
pub struct SensorListenerState {
    reader: EventReader<Manifold>,
}

pub fn character_controller_system(
    time: Res<Time>,
    mut active: ResMut<ActiveCameras>,
    input: Res<Input<KeyCode>>,
    mouse: Res<Events<MouseMotion>>,
    mut players: Query<(Mut<Character>, Mut<RigidBody>)>,
    mut cameras: Query<With<Camera, Mut<Transform>>>,
    mut fp: Query<With<FirstPersonCamera, (Entity, Mut<Camera>)>>,
    mut debug: Query<With<FlyCamera, (Entity, Mut<Camera>, Mut<Transform>)>>,
) {
    let delta_time = time.delta.as_secs_f32();
    for (mut controller, mut body) in &mut players.iter() {
        if input.just_pressed(KeyCode::G) {
            controller.active = !controller.active;
            if controller.active {
                for (e, mut camera) in &mut fp.iter() {
                    active.cameras.insert("Camera3d".to_string(), Some(e));
                    camera.name = Some("Camera3d".to_string());
                }
                for (_, mut camera, _) in &mut debug.iter() {
                    camera.name = Some("None".to_string());
                }
            } else {
                for (e, mut camera, mut transform) in &mut debug.iter() {
                    transform.set_translation(Vec3::new(body.position.x(), 1.6, body.position.y()));
                    transform.set_rotation(Quat::from_rotation_ypr(
                        controller.yrot,
                        controller.xrot,
                        0.0,
                    ));
                    active.cameras.insert("Camera3d".to_string(), Some(e));
                    camera.name = Some("Camera3d".to_string());
                }
                for (_, mut camera) in &mut fp.iter() {
                    camera.name = Some("None".to_string());
                }
            }
        }

        if !controller.active {
            continue;
        }
        let mut yrot = 0.0;
        let mut xrot = 0.0;
        for motion in controller.reader.iter(&mouse) {
            yrot -= motion.delta.x() * delta_time * MOUSE_SPEED;
            xrot -= motion.delta.y() * delta_time * MOUSE_SPEED;
        }
        controller.yrot += yrot;
        controller.xrot += xrot;
        if let Some(e) = active.get(CAMERA3D) {
            let mut camera = cameras.get_mut::<Transform>(e).unwrap();
            camera.set_rotation(Quat::from_rotation_ypr(
                controller.yrot,
                controller.xrot,
                0.0,
            ));
        }
        let mut addvel = Vec2::new(0.0, 0.0);
        if input.pressed(KeyCode::W) {
            *addvel.y_mut() -= INC_SPEED;
        }
        if input.pressed(KeyCode::S) {
            *addvel.y_mut() += INC_SPEED;
        }
        if input.pressed(KeyCode::A) {
            *addvel.x_mut() -= INC_SPEED;
        }
        if input.pressed(KeyCode::D) {
            *addvel.x_mut() += INC_SPEED;
        }

        let rot = Mat2::from_angle(-controller.yrot);
        body.velocity += rot * addvel * delta_time;

        if body.velocity.length() > MAX_SPEED {
            body.velocity = body.velocity.normalize() * MAX_SPEED;
        }
    }
}

pub fn sensor_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    events: Res<Events<Manifold>>,
    mut state: ResMut<SensorListenerState>,
    mut current: ResMut<CurrentRoom>,
    mut active: Query<With<ActiveRoom, Entity>>,
    mut sensor: Query<&Sensor>,
    mut connection: Query<Mut<Connection>>,
    mut bodies: Query<Mut<RigidBody>>,
    mut room_sensors: Query<&RoomSensor>,
    mut players: Query<(Entity, Mut<Character>)>,
) {
    for manifold in state.reader.iter(&events) {
        if players.get::<Character>(manifold.a).is_ok() {
            if let Ok(room) = room_sensors.get::<RoomSensor>(manifold.b) {
                for e in &mut active.iter() {
                    commands.remove_one::<ActiveRoom>(e);
                }

                let mut body = bodies.get_mut::<RigidBody>(manifold.b).unwrap();
                body.set_active(false);
                mem::drop(body);

                current.entity = room.0;
                let body = bodies.get::<RigidBody>(room.0).unwrap();
                let position = body.position;
                let rotation = body.rotation;
                mem::drop(body);
                for (e, mut controller) in &mut players.iter() {
                    let mut body = bodies.get_mut::<RigidBody>(e).unwrap();
                    body.position -= position;
                    let rot = Mat2::from_angle(rotation);
                    body.position = rot * body.position;
                    body.velocity = rot * body.velocity;
                    controller.yrot -= rotation;
                }
            }
        } else if players.get::<Character>(manifold.b).is_ok() {
            if let Ok(room) = room_sensors.get::<RoomSensor>(manifold.a) {
                for e in &mut active.iter() {
                    commands.remove_one::<ActiveRoom>(e);
                }

                let mut body = bodies.get_mut::<RigidBody>(manifold.a).unwrap();
                body.set_active(false);
                mem::drop(body);

                current.entity = room.0;
                let body = bodies.get::<RigidBody>(room.0).unwrap();
                let position = body.position;
                let rotation = body.rotation;
                mem::drop(body);
                for (e, mut controller) in &mut players.iter() {
                    let mut body = bodies.get_mut::<RigidBody>(e).unwrap();
                    body.position -= position;
                    let rot = Mat2::from_angle(rotation);
                    body.position = rot * body.position;
                    body.velocity = rot * body.velocity;
                    controller.yrot -= rotation;
                }
            }
        }
        if sensor.get::<Sensor>(manifold.a).is_ok() {
            if let Ok(mut conn) = connection.get_mut::<Connection>(manifold.b) {
                if input.just_pressed(KeyCode::Space) {
                    let mut body = bodies.get_mut::<RigidBody>(manifold.b).unwrap();
                    conn.open = !conn.open;
                    if !conn.open {
                        body.rotation -= 90.0_f32.to_radians();
                        let rot = Mat2::from_angle(body.rotation);
                        let offset = rot * Vec2::new(0.5, 0.5);
                        body.position -= offset;
                        body.set_sensor(false);
                    } else {
                        let rot = Mat2::from_angle(body.rotation);
                        let offset = rot * Vec2::new(0.5, 0.5);
                        body.position += offset;
                        body.rotation += 90.0_f32.to_radians();
                        body.set_sensor(true);
                    }

                    mem::drop(body);
                    bodies
                        .get_mut::<RigidBody>(conn.sensor)
                        .unwrap()
                        .set_active(conn.open);
                }
            }
        } else if sensor.get::<Sensor>(manifold.b).is_ok() {
            if let Ok(mut conn) = connection.get_mut::<Connection>(manifold.a) {
                if input.just_pressed(KeyCode::Space) {
                    let mut body = bodies.get_mut::<RigidBody>(manifold.a).unwrap();
                    conn.open = !conn.open;
                    if !conn.open {
                        body.rotation -= 90.0_f32.to_radians();
                        let rot = Mat2::from_angle(body.rotation);
                        let offset = rot * Vec2::new(0.5, 0.5);
                        body.position -= offset;
                        body.set_sensor(false);
                    } else {
                        let rot = Mat2::from_angle(body.rotation);
                        let offset = rot * Vec2::new(0.5, 0.5);
                        body.position += offset;
                        body.rotation += 90.0_f32.to_radians();
                        body.set_sensor(true);
                    }

                    mem::drop(body);
                    bodies
                        .get_mut::<RigidBody>(conn.sensor)
                        .unwrap()
                        .set_active(conn.open);
                }
            }
        }
    }
}
