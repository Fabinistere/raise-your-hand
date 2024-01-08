//! Implements Npc for moving and steering entities.

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::characters::movement::Speed;

use super::{Friend, NPC};

// pub const PROXIMITY_RADIUS: f32 = 64.;

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

pub fn friend_movement(
    mut friend_query: Query<
        (
            Entity,
            // &mut NPCBehavior,
            &mut Velocity,
        ),
        (With<NPC>, With<Friend>),
    >,
) {
    if let Ok((friend, mut rb_vel)) = friend_query.get_single_mut() {
        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;
    }
}

pub fn npc_movement(
    mut npc_query: Query<
        (
            Entity,
            // &mut NPCBehavior,
            &Transform,
            &Speed,
            &mut Velocity,
            &Name,
        ),
        (With<NPC>, Without<Friend>),
    >,
    pos_query: Query<&GlobalTransform>,
) {
    for (npc, transform, speed, mut rb_vel, npc_name) in &mut npc_query {}
}

/// Return velocity x and y value to move forward a certain target
fn move_to(target_transform: &GlobalTransform, transform: &Transform, speed: &Speed) -> (f32, f32) {
    // REFACTOR: use the max_step possible and see if the difference can be lowered.

    let up = target_transform.translation().y > transform.translation.y;
    let down = target_transform.translation().y < transform.translation.y;
    let left = target_transform.translation().x < transform.translation.x;
    let right = target_transform.translation().x > transform.translation.x;

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    // println!("x: {}, y: {}", x_axis, y_axis);

    let mut vel_x = x_axis as f32 * **speed;
    let mut vel_y = y_axis as f32 * **speed;

    if x_axis != 0 && y_axis != 0 {
        vel_x *= (std::f32::consts::PI / 4.).cos();
        vel_y *= (std::f32::consts::PI / 4.).cos();
    }

    (vel_x, vel_y)
}
