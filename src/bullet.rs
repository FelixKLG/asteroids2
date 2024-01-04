use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, ttl::TimeToLive};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot);
    }
}

#[derive(Component)]
pub struct Bullet;

fn shoot(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut characters: Query<(&Transform, With<Player>)>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (transform, _) in characters.iter_mut() {
            let bullet_sprite = asset_server.load("Bullet.png");

            let mut spawn_pos = *transform;

            spawn_pos.translation.y += 30.0;

            commands
                .spawn(Bullet)
                .insert(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(30., 40.)),
                        ..Default::default()
                    },
                    transform: spawn_pos,
                    texture: bullet_sprite,
                    ..Default::default()
                })
                .insert(Name::new("Bullet"))
                .insert(TimeToLive::new(Timer::new(
                    Duration::from_secs(3),
                    TimerMode::Once,
                )))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Collider::ball(15.0))
                .insert(Velocity::linear(Vec2::new(0.0, 300.)))
                .insert(RigidBody::Dynamic);
        }
    }
}
