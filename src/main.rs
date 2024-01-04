use bevy::{app::AppExit, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod asteroids;
mod bullet;
mod collisions;
mod player;
mod setup;
mod ttl;

#[derive(Resource)]
struct GameData {
    pub score: u32,
    pub asteroids: u32,
    pub lives: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            asteroids: 0,
            lives: 10,
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
        .add_plugins(setup::SetupPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(asteroids::AsteroidPlugin)
        .add_plugins(ttl::TimeToLivePlugin)
        .add_plugins(collisions::ColsPlugin)
        .add_systems(Update, (game_over, game_won))
        // TODO: REMOVE THESE BEFORE RELEASE BUILD!
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<GameData>()
        .run();
}

fn game_over(game_state: ResMut<GameData>, mut exit_events: EventWriter<AppExit>) {
    if game_state.lives <= 0 {
        info!("Game Over!");
        exit_events.send(AppExit);
    }
}

fn game_won(game_state: ResMut<GameData>, mut exit_events: EventWriter<AppExit>) {
    if game_state.score >= 5 {
        info!("You Won!");
        exit_events.send(AppExit);
    }
}
