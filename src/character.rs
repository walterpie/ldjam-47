use bevy::math::*;
use bevy::prelude::*;

use crate::phys::*;
use crate::proc::Connection;

pub const MAX_SPEED: f32 = 2.5;
pub const INC_SPEED: f32 = 5.0;

#[derive(Default, Debug)]
pub struct Character;

#[derive(Default, Debug)]
pub struct Sensor;

#[derive(Bundle)]
pub struct CharBundle {
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
    input: Res<Input<KeyCode>>,
    mut players: Query<With<Character, Mut<RigidBody>>>,
) {
    let delta_time = time.delta.as_secs_f32();
    for mut body in &mut players.iter() {
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

        body.velocity += addvel * delta_time;

        if body.velocity.length() > MAX_SPEED {
            body.velocity = body.velocity.normalize() * MAX_SPEED;
        }
    }
}

pub fn sensor_system(
    input: Res<Input<KeyCode>>,
    events: Res<Events<Manifold>>,
    mut state: ResMut<SensorListenerState>,
    mut sensor: Query<&Sensor>,
    mut connection: Query<(Mut<Connection>, Mut<RigidBody>)>,
) {
    for manifold in state.reader.iter(&events) {
        if sensor.get::<Sensor>(manifold.a).is_ok() {
            if let Ok(mut conn) = connection.get_mut::<Connection>(manifold.b) {
                if input.just_pressed(KeyCode::Space) {
                    let mut body = connection.get_mut::<RigidBody>(manifold.b).unwrap();
                    if conn.open {
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
                    conn.open = !conn.open;
                }
            }
        } else if sensor.get::<Sensor>(manifold.b).is_ok() {
            if let Ok(mut conn) = connection.get_mut::<Connection>(manifold.a) {
                if input.just_pressed(KeyCode::Space) {
                    let mut body = connection.get_mut::<RigidBody>(manifold.a).unwrap();
                    if conn.open {
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
                    conn.open = !conn.open;
                }
            }
        }
    }
}
