use bevy::math::*;
use bevy::prelude::*;

use crate::phys::*;

pub const MAX_SPEED: f32 = 2.5;
pub const INC_SPEED: f32 = 5.0;

#[derive(Default, Debug)]
pub struct Character;

#[derive(Bundle)]
pub struct CharBundle {
    pub controller: Character,
    pub body: RigidBody,
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
