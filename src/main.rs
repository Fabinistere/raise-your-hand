#![allow(clippy::type_complexity)]

use bevy::{asset::ChangeWatcher, ecs::schedule::ScheduleBuildSettings, prelude::*};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Template  Bevy".to_string(),
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        // vsync: true,
                        ..Window::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            // bevy_tweening::TweeningPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // ----- Our plugins -----
        ))
        .add_systems(Startup, game_setup);

    app.edit_schedule(Main, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: bevy::ecs::schedule::LogLevel::Warn,
            ..default()
        });
    });

    app.run();
}

#[derive(Component)]
struct PlayerCamera;

fn game_setup(
    mut commands: Commands,
    // mut rapier_config: ResMut<RapierConfiguration>
) {
    // rapier_config.gravity = Vect::ZERO;

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.1;
    commands.spawn((camera, PlayerCamera));
}
