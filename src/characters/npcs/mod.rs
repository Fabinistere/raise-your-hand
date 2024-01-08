//! NPCs lockup

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::character::{npcs::NPC_SCALE, CHAR_HITBOX_WIDTH},
    GameState,
};

use super::{movement::MovementBundle, CharacterHitbox};

pub mod movement;

pub struct NPCPlugin;

/**
 * NPC has hobbies
 *  - landwark
 *    - index in const, with free: bol
 *    - when talking to a npc in a landwark, include the other present
 *    -> rest
 *  - stroll
 *    - in a restricted zone -index in const-
 *    -> rest
 *  - rest
 *    -> stroll
 *    -> landwark
 *  - talking to MC
 *    - infite rest until the MC is leaving
 *    -> short rest
 *    or
 *    -> stroll
 *    -> landmark
 *    -> rest
 *
 * Reflexion
 *  - should npc avoid hit other entity
 *  - turn false the free param from a landmark position taken by the MC
 */
impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_friend, spawn_walkers))
            .add_systems(Update, movement::friend_movement);
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct Friend;

#[derive(Component)]
pub struct NPCHitbox;

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

fn spawn_walkers(mut commands: Commands) {}

fn spawn_friend(mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::from((10., 10., 0.)),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("The Friend"),
            NPC,
            Friend,
            // -- Animation --
            MovementBundle::default(),
            // -- Hitbox --
            RigidBody::Dynamic,
            // 10 = Cannot be moved by anything
            // Dominance::group(10),
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(CHAR_HITBOX_WIDTH),
                // Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                NPCHitbox,
                CharacterHitbox,
                Name::new("Friend Hitbox"),
            ));
        });
}
