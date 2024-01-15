//! Implements Npc for moving and steering entities.

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::Rng;

use crate::characters::movement::Speed;

use super::{Friend, Target, Walker};

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
        With<Friend>,
    >,
) {
    if let Ok((_friend, mut rb_vel)) = friend_query.get_single_mut() {
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
            &mut Target,
            &Name,
        ),
        With<Walker>,
    >,
) {
    for (_npc, transform, speed, mut rb_vel, mut target_transform, _npc_name) in &mut npc_query {
        let is_close = (target_transform.translation.x <= transform.translation.x + 1.
            && target_transform.translation.x >= transform.translation.x - 1.)
            && (target_transform.translation.y <= transform.translation.y + 1.
                && target_transform.translation.y >= transform.translation.y - 1.);
        if is_close {
            target_transform.0 = new_transform();
        } else {
            let (vel_x, vel_y) = move_to(&target_transform, transform, speed);

            rb_vel.linvel.x = vel_x;
            rb_vel.linvel.y = vel_y;
        }
    }
}

/// Return velocity x and y value to move forward a certain target entity
fn move_to(target_transform: &Transform, transform: &Transform, speed: &Speed) -> (f32, f32) {
    // REFACTOR: use the max_step possible and see if the difference can be lowered.

    let up = target_transform.translation.y > transform.translation.y;
    let down = target_transform.translation.y < transform.translation.y;
    let left = target_transform.translation.x < transform.translation.x;
    let right = target_transform.translation.x > transform.translation.x;

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

pub fn new_transform() -> Transform {
    let x = rand::thread_rng().gen_range(-50..50);
    let y = rand::thread_rng().gen_range(-50..50);
    return Transform::from_xyz(x as f32, y as f32, 0.);
}
