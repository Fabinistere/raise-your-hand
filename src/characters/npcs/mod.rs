//! NPCs lockup

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::character::{npcs::NPC_SCALE, CHAR_HITBOX_WIDTH},
    level::Level,
    GameState,
};

use self::movement::{Direction, DodgeMeasure, FrontSensor};

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
        app.add_systems(OnEnter(GameState::Init), (spawn_friend, spawn_walkers))
            .add_systems(
                Update,
                (
                    movement::dodge_measure.before(movement::walker_movement),
                    movement::walker_movement,
                    movement::friend_movement,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct Walker;

/// TODO: Change to Entity or Path
///
/// TODO: feature - pathfinding to reach the entity
#[derive(Reflect, Deref, DerefMut, Component)]
pub struct Target(pub Transform);

#[derive(Component)]
pub struct WalkerHitbox;

#[derive(Component)]
pub struct Friend;

#[derive(Component)]
pub struct FriendHitbox;

/* -------------------------------------------------------------------------- */
/*                                  Spawners                                  */
/* -------------------------------------------------------------------------- */

fn spawn_walkers(mut commands: Commands, level: Res<Level>) {
    for i in 0..(level.difficulty * 10) {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::from((10. * i as f32, 20. * i as f32, 0.)),
                        scale: Vec3::splat(NPC_SCALE),
                        ..default()
                    },
                    ..default()
                },
                Name::new(format!("Walker {}", i)),
                Walker,
                // -- Movement --
                Target(movement::new_place_to_go()),
                DodgeMeasure::default(),
                Direction::default(),
                // -- Animation --
                MovementBundle::default(),
                // -- Hitbox --
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Collider::ball(CHAR_HITBOX_WIDTH),
                    // Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                    WalkerHitbox,
                    CharacterHitbox,
                    Name::new(format!("Walker {} Hitbox", i)),
                ));

                parent.spawn((
                    Collider::cuboid(3., 3.5),
                    Transform::from_xyz(0., 5., 0.),
                    FrontSensor,
                    Sensor,
                    Name::new(format!("Walker {} Front Sensor", i)),
                ));
            });
    }
}

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
                FriendHitbox,
                CharacterHitbox,
                ActiveEvents::COLLISION_EVENTS,
                Name::new("Friend Hitbox"),
            ));
        });
}
