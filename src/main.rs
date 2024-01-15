#![allow(clippy::type_complexity)]

use bevy::{ecs::schedule::ScheduleBuildSettings, prelude::*};
use bevy_rapier2d::prelude::*;
use controls::Key;

mod characters;
pub mod collisions;
pub mod constants;
pub mod controls;
mod debug;
mod level;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(controls::KeyBindings {
            up: [Key(KeyCode::W), Key(KeyCode::Z), Key(KeyCode::Up)],
            down: [Key(KeyCode::S), Key(KeyCode::Down)],
            right: [Key(KeyCode::D), Key(KeyCode::Right)],
            left: [Key(KeyCode::A), Key(KeyCode::Q), Key(KeyCode::Left)],
        })
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Raise your hand!".to_string(),
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        // vsync: true,
                        ..Window::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin::default()),
            // bevy_tweening::TweeningPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // ----- Our plugins -----
            level::LevelPlugin,
            characters::CharactersPlugin,
            debug::DebugPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, game_setup);

    app.edit_schedule(Main, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: bevy::ecs::schedule::LogLevel::Warn,
            ..default()
        });
    });

    app.run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum GameState {
    TitleScreen,
    #[default]
    Init,
    Playing,
}

#[derive(Component)]
struct PlayerCamera;

fn game_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vect::ZERO;

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.1;
    commands.spawn((camera, PlayerCamera));
}
