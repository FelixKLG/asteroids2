use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod asteroids;
mod bullet;
mod player;
mod setup;

#[derive(Resource)]
struct GameData {
    pub score: u32,
    pub asteroids: u32,
    pub lives: u32,
    pub game_over: bool,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            asteroids: 0,
            lives: 3,
            game_over: false,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Asteroids".into(),
                        resizable: false,
                        resolution: WindowResolution::new(800., 800.),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_linear()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(setup::SetupPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(asteroids::AsteroidPlugin)
        .init_resource::<GameData>()
        .run()
}