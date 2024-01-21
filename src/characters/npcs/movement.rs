//! Implements Npc for moving and steering entities.

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::{pipeline::CollisionEvent, prelude::Velocity};
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    characters::{movement::Speed, CharacterHitbox},
    collisions::CollisionEventExt,
};

use super::{Friend, Target, Walker};

// pub const PROXIMITY_RADIUS: f32 = 64.;

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

/// Relative to the (x, y) of `GlobalTransform`
#[derive(Reflect, Component, Default)]
pub enum Direction {
    #[default]
    Top,
    TopLeft,
    TopRight,
    Left,
    Right,
    BotLeft,
    BotRight,
    Bot,
}

impl Direction {
    pub fn from_binaries(up: bool, down: bool, left: bool, right: bool) -> Option<Self> {
        match (up as i8, down as i8, left as i8, right as i8) {
            (1, 0, 0, 0) => Some(Direction::Top),
            (0, 1, 0, 0) => Some(Direction::Bot),
            (0, 0, 1, 0) => Some(Direction::Left),
            (0, 0, 0, 1) => Some(Direction::Right),
            (1, 0, 0, 1) => Some(Direction::TopRight),
            (1, 0, 1, 0) => Some(Direction::TopLeft),
            (0, 1, 0, 1) => Some(Direction::BotRight),
            (0, 1, 1, 0) => Some(Direction::BotLeft),
            _ => None,
        }
    }

    pub fn to_angle(&self) -> f32 {
        match self {
            Direction::Top => 0.,
            Direction::Bot => PI,
            Direction::Left => PI / 2.,
            Direction::Right => 3. * PI / 2.,
            Direction::TopLeft => PI / 4.,
            Direction::TopRight => 7. * PI / 4., // -PI / 4.
            Direction::BotLeft => 3. * PI / 4.,  // 3. * PI / 4.
            Direction::BotRight => 5. * PI / 4., // -3. * PI / 4.
        }
    }
}

#[derive(Component)]
pub struct FrontSensor;

#[derive(Component, Default)]
pub struct DodgeMeasure {
    is_dodging: bool,
    dodging_entities: HashSet<Entity>,
}

impl DodgeMeasure {
    pub fn is_dodging(&self) -> bool {
        self.is_dodging
    }

    /// If there is any entity in front of the walker,
    /// `[is_dodging()]` will return true.
    pub fn add_danger_to_dodge(&mut self, entity: Entity) {
        self.dodging_entities.insert(entity);
        self.is_dodging = true;
    }

    pub fn remove_danger_to_dodge(&mut self, entity: Entity) {
        self.dodging_entities.remove(&entity);
        self.is_dodging = !self.dodging_entities.is_empty();
    }
}

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

/// Collisions detector between the walker and something else
///
/// - Detect when an entity enters their `[raise-your-hand::characters::npcs::movement::FrontSensor]`
/// The walker will deviate to the right.
pub fn dodge_measure(
    mut collision_events: EventReader<CollisionEvent>,
    character_hitboxes_query: Query<&Parent, With<CharacterHitbox>>,
    walker_front_sensors_query: Query<&Parent, With<FrontSensor>>,

    mut walkers_query: Query<&mut DodgeMeasure, With<Walker>>,
) {
    for collision_event in collision_events.read() {
        // info!("{:#?}", collision_event);
        let (e1, e2) = collision_event.entities();

        if let (Ok(incoming_danger), Err(_), Err(_), Ok(walker))
        | (Err(_), Ok(incoming_danger), Ok(walker), Err(_)) = (
            character_hitboxes_query.get(e1),
            character_hitboxes_query.get(e2),
            walker_front_sensors_query.get(e1),
            walker_front_sensors_query.get(e2),
        ) {
            if let Ok(mut dodge_measure) = walkers_query.get_mut(**walker) {
                // FIXME: danger avoidance

                if collision_event.is_started() {
                    // info!("danger detected");
                    dodge_measure.add_danger_to_dodge(**incoming_danger);
                } else if collision_event.is_stopped() {
                    // info!("danger avoided");
                    dodge_measure.remove_danger_to_dodge(**incoming_danger);
                }
            }
        }
    }
}

/// Move the walker to their destination
/// and rotate them to face their direction.
pub fn walker_movement(
    mut npc_query: Query<
        (
            Entity,
            // &mut NPCBehavior,
            &mut Transform, // `mut` to change the rotation
            &Speed,
            &mut Velocity,
            &mut Target,
            &DodgeMeasure,
            &mut Direction,
            &Name,
        ),
        With<Walker>,
    >,
) {
    for (
        _npc,
        mut transform,
        speed,
        mut rb_vel,
        mut target_transform,
        dodging_measure,
        mut direction,
        _npc_name,
    ) in &mut npc_query
    {
        let is_close = (target_transform.translation.x <= transform.translation.x + 1.
            && target_transform.translation.x >= transform.translation.x - 1.)
            && (target_transform.translation.y <= transform.translation.y + 1.
                && target_transform.translation.y >= transform.translation.y - 1.);
        if is_close {
            target_transform.0 = new_place_to_go();
        } else {
            let (mut vel_x, mut vel_y, potential_new_direction) =
                move_to(&target_transform, &transform, speed);

            if let Some(new_dir) = potential_new_direction {
                *direction = new_dir;
            }

            // Force dodge over goal shortest path
            // The walker will dodge by *their* right
            if dodging_measure.is_dodging() {
                let running_vel = 1. * **speed * (PI / 4.).cos();

                // The direction is also changed
                (vel_x, vel_y, *direction) = match *direction {
                    Direction::Top => (running_vel, 0., Direction::Right),
                    Direction::Bot => (-running_vel, 0., Direction::Left),
                    Direction::Left => (0., running_vel, Direction::Top),
                    Direction::Right => (0., -running_vel, Direction::Bot),
                    // hard dodge right
                    Direction::TopLeft => (-vel_x, vel_y, Direction::TopRight),
                    Direction::TopRight => (vel_x, -vel_y, Direction::BotRight), // = (vel_x, 0.)
                    Direction::BotLeft => (vel_x, -vel_y, Direction::TopLeft),   // = (vel_x, 0.)
                    Direction::BotRight => (-vel_x, vel_y, Direction::Bot),
                };
            }

            // Rotate the walker to place the FrontSensor correctly
            // The walker looks where it's going
            let angle = direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle);

            rb_vel.linvel.x = vel_x;
            rb_vel.linvel.y = vel_y;
        }
    }
}

/// Return velocity x and y value to move forward a certain target entity
/// and the relative direction.
fn move_to(
    target_transform: &Transform,
    transform: &Transform,
    speed: &Speed,
) -> (f32, f32, Option<Direction>) {
    // REFACTOR: use the max_step possible and see if the difference can be lowered.
    // Is the step needed is high enough to avoid any tremble
    let up_step_worthit = (target_transform.translation.y - transform.translation.y).abs() > 0.5;
    let down_step_worthit = (transform.translation.y - target_transform.translation.y).abs() > 0.5;
    let left_step_worthit = (target_transform.translation.x - transform.translation.x).abs() > 0.5;
    let right_step_worthit = (transform.translation.x - target_transform.translation.x).abs() > 0.5;

    // Is there a need to go up (resp. down, left, right) ?
    let up = up_step_worthit && target_transform.translation.y > transform.translation.y;
    let down = down_step_worthit && target_transform.translation.y < transform.translation.y;
    let left = left_step_worthit && target_transform.translation.x < transform.translation.x;
    let right = right_step_worthit && target_transform.translation.x > transform.translation.x;

    let potential_direction = Direction::from_binaries(up, down, left, right);

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    // println!("x: {}, y: {}", x_axis, y_axis);

    let mut vel_x = x_axis as f32 * **speed;
    let mut vel_y = y_axis as f32 * **speed;

    if x_axis != 0 && y_axis != 0 {
        vel_x *= (std::f32::consts::PI / 4.).cos();
        vel_y *= (std::f32::consts::PI / 4.).cos();
    }

    (vel_x, vel_y, potential_direction)
}

pub fn new_place_to_go() -> Transform {
    let x = rand::thread_rng().gen_range(-50..50);
    let y = rand::thread_rng().gen_range(-50..50);
    return Transform::from_xyz(x as f32, y as f32, 0.);
}
