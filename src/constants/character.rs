pub const CHAR_SCALE: f32 = 0.6 * super::TILE_SIZE;

pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;

pub mod player {
    use crate::constants::TILE_SIZE;

    pub const PLAYER_WIDTH: f32 = 12.;
    pub const PLAYER_HEIGHT: f32 = 15.;
    pub const PLAYER_SCALE: f32 = super::CHAR_SCALE;
    pub const PLAYER_SPEED: f32 = 75. * TILE_SIZE;
    // pub const PLAYER_SPAWN: (f32, f32, f32) = (-24., -150., 0.);

    pub const CAMERA_INTERPOLATION: f32 = 0.1;
}

pub mod npcs {
    pub const NPC_SCALE: f32 = super::CHAR_SCALE;

    pub mod movement {
        use crate::constants::TILE_SIZE;

        pub const REST_TIMER: u64 = 3;

        pub const NPC_SPEED: f32 = 50. * TILE_SIZE; // -> Speed::default()
    }
}
