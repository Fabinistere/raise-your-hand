pub mod movement;
pub mod npcs;
pub mod player;

use bevy::prelude::*;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, npcs::NPCPlugin));
    }
}

#[derive(Component)]
pub struct CharacterHitbox;
