use std::mem;

use bevy::prelude::*;
use bevy_rapier3d::physics::*;
use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::dynamics::RigidBodySet;
use rapier3d::geometry::ColliderBuilder;
use rapier3d::math::*;
use rapier3d::na::UnitQuaternion;

pub const MAX_SPEED: f32 = 1.5;
pub const INC_SPEED: f32 = 3.0;
pub const LIN_DAMP: f32 = 0.9;

#[derive(Default, Debug)]
pub struct CharacterDynamics;

#[derive(Default, Debug)]
pub struct Character {
    linvel: Vector<f32>,
    y: f32,
}

#[derive(Bundle)]
pub struct CharBundle {
    pub controller: Character,
    pub body: RigidBodyBuilder,
    pub collider: ColliderBuilder,
}

pub fn character_controller_system(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut bodies: ResMut<RigidBodySet>,
    mut players: Query<(Mut<Character>, &RigidBodyHandleComponent)>,
    mut dynamics: Query<With<CharacterDynamics, &RigidBodyHandleComponent>>,
) {
    let delta_time = time.delta.as_secs_f32();
    for (mut controller, handle) in &mut players.iter() {
        let handle = handle.handle();

        let mut addvel = Vector::new(0.0, 0.0, 0.0);
        if input.pressed(KeyCode::W) {
            addvel.z -= INC_SPEED;
        }
        if input.pressed(KeyCode::S) {
            addvel.z += INC_SPEED;
        }
        if input.pressed(KeyCode::A) {
            addvel.x -= INC_SPEED;
        }
        if input.pressed(KeyCode::D) {
            addvel.x += INC_SPEED;
        }

        controller.linvel += addvel * delta_time;

        if controller.linvel.magnitude() > MAX_SPEED {
            controller.linvel.set_magnitude(MAX_SPEED);
        }

        if addvel.magnitude_squared() <= f32::EPSILON {
            if controller.linvel.magnitude_squared() > f32::EPSILON {
                controller.linvel *= LIN_DAMP;
            }
        }

        for dynamic in &mut dynamics.iter() {
            let dynamic = dynamic.handle();
            let dynamic = bodies.get(dynamic).unwrap();

            controller.y = dynamic.position.translation.y;
        }

        let change = controller.linvel * delta_time;
        let mut body = bodies.get_mut(handle).unwrap();
        let mut new = body.position * Translation::from(change);
        new.translation.y = controller.y;
        body.set_next_kinematic_position(new);
        mem::drop(body);

        for dynamic in &mut dynamics.iter() {
            let dynamic = dynamic.handle();
            let mut dynamic = bodies.get_mut(dynamic).unwrap();

            dynamic.position.translation.x = new.translation.x;
            dynamic.position.translation.z = new.translation.z;
            dynamic.position.rotation = UnitQuaternion::identity();
        }
    }
}
