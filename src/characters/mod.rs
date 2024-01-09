pub mod movement;
pub mod npcs;
pub mod player;

use bevy::prelude::*;
use bevy_rapier2d::pipeline::CollisionEvent;

use crate::collisions::CollisionEventExt;

use self::{npcs::FriendHitbox, player::PlayerHitbox};

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, npcs::NPCPlugin))
            .add_systems(Update, friends_reunited);
    }
}

#[derive(Component)]
pub struct CharacterHitbox;

fn friends_reunited(
    mut collision_events: EventReader<CollisionEvent>,
    friend_hitbox_query: Query<&Parent, With<FriendHitbox>>,
    player_hitbox_query: Query<Entity, With<PlayerHitbox>>,
) {
    for collision_event in collision_events.read() {
        // info!("{:#?}", collision_event);
        if collision_event.is_started() {
            let (e1, e2) = collision_event.entities();

            if let (Ok(_friend), Err(_), Err(_), Ok(_player))
            | (Err(_), Ok(_friend), Ok(_player), Err(_)) = (
                friend_hitbox_query.get(e1),
                friend_hitbox_query.get(e2),
                player_hitbox_query.get(e1),
                player_hitbox_query.get(e2),
            ) {
                info!("WIN!");
                // TODO: restart with *2 walkers number
            }
        }
    }
}
