use bevy::prelude::*;

use crate::{
    characters::{
        npcs::{Friend, Walker},
        player::Player,
    },
    GameState,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::default())
            .add_systems(Update, next_level.run_if(in_state(GameState::Init)))
            .add_systems(OnExit(GameState::Playing), clean_level);
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Resource)]
pub struct Level {
    pub difficulty: i32,
    // number: i32,
}

impl Default for Level {
    fn default() -> Self {
        Level { difficulty: 1 }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

fn clean_level(
    mut commands: Commands,
    living_beings_query: Query<Entity, Or<(With<Player>, With<Walker>, With<Friend>)>>,
    // hitboxes_query: Query<Entity, Or<(With<PlayerHitbox>, With<WalkerHitbox>, With<FriendHitbox>)>>
) {
    for living_being in &living_beings_query {
        commands.entity(living_being).despawn_recursive();
    }
}

fn next_level(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Playing);
}
