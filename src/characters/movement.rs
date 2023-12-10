use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
// use bevy_retrograde::prelude::Velocity;

use crate::constants::TILE_SIZE;

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(50. * TILE_SIZE)
    }
}

#[derive(Default, Bundle)]
pub struct MovementBundle {
    pub speed: Speed,
    pub velocity: Velocity,
}

impl MovementBundle {
    pub fn new(speed: f32) -> Self {
        MovementBundle {
            speed: Speed(speed),
            velocity: Velocity::zero(),
        }
    }
}

#[derive(Component)]
pub struct CharacterCloseSensor;