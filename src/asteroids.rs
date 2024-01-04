use crate::GameData;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_asteroids, oob_kill));
    }
}

#[derive(Component)]
pub struct Asteroid;

fn spawn_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_data: ResMut<GameData>,
) {
    let mut rng = ThreadRng::default();

    let texture1 = asset_server.load("Asteroid.png");
    let texture2 = asset_server.load("Asteroid 2.png");
    let texture3 = asset_server.load("Asteroid 3.png");

    let pick_texture = |x| match x {
        0 => texture1,
        1 => texture2,
        2 => texture3,
        _ => unreachable!("Invalid texture index"),
    };

    if game_data.asteroids < 5 {
        let random_x = rng.gen_range(-350.0..350.0);
        let random_y = rng.gen_range(-50.0..300.0);
        let negative_y_velocity = rng.gen_range(-10.0..=-5.0);
        let gravity_scale = rng.gen_range(0.1..=0.6);

        commands
            .spawn(Asteroid)
            .insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(60., 60.)),
                    ..Default::default()
                },
                texture: pick_texture(rng.gen_range(0..=2)),
                transform: Transform {
                    translation: Vec3::new(random_x, random_y, 0.),
                    rotation: Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::PI)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::ball(30.0))
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(gravity_scale))
            .insert(Velocity::linear(Vec2::new(0.0, negative_y_velocity)))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Name::from("Asteroid"));

        game_data.asteroids += 1;
    }
}

fn oob_kill(
    mut commands: Commands,
    mut asteroids: Query<(Entity, &Transform), With<Asteroid>>,
    mut game_data: ResMut<GameData>,
) {
    for (asteroid, transform) in asteroids.iter_mut() {
        if transform.translation.y < -500.0 {
            commands.entity(asteroid).despawn();
            game_data.asteroids -= 1;
        }
    }
}
