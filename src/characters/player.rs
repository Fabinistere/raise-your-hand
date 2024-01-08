use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        movement::{MovementBundle, Speed},
        CharacterHitbox,
    },
    constants::character::{player::*, *},
    controls::KeyBindings,
    GameState, PlayerCamera,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (player_movement, camera_follow));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHitbox;

#[derive(Component)]
struct Immobilized;

#[derive(Component)]
pub struct PlayerInteractionSensor;

#[derive(Component)]
pub struct PlayerCloseSensor;

fn player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Speed, &mut Velocity), With<Player>>,
) {
    if let Ok((_player, speed, mut rb_vel)) = player_query.get_single_mut() {
        let up = keyboard_input.any_pressed(key_bindings.up());
        let down = keyboard_input.any_pressed(key_bindings.down());
        let left = keyboard_input.any_pressed(key_bindings.left());
        let right = keyboard_input.any_pressed(key_bindings.right());

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
            vel_y *= (std::f32::consts::PI / 4.).cos();
        }

        // rb_vel.linvel.x = x_axis as f32 * **speed * 200. * time.delta_seconds();
        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;
    }
}

/// ## Notes
///
/// TODO: integrate a camera follow that get stuck into the corners of the map (to visualize them better)
fn camera_follow(
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    if let Ok(t) = query.p0().get_single() {
        let player_transform = *t;

        if let Ok(mut camera_transform) = query.p1().get_single_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z,
                ),
                CAMERA_INTERPOLATION,
            );
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
                ..default()
            },
            Name::new("Player"),
            Player,
            // -- Animation --
            MovementBundle::new(PLAYER_SPEED),
            // -- Hitbox --
            RigidBody::Dynamic,
            // 10 = Cannot be moved by anything
            // Dominance::group(1),
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(CHAR_HITBOX_WIDTH),
                // Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                PlayerHitbox,
                CharacterHitbox,
                Name::new("Player Hitbox"),
            ));
        });
}
