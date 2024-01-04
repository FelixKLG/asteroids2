use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{asteroids::Asteroid, bullet::Bullet, GameData};

pub struct ColsPlugin;

impl Plugin for ColsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bullet_asteroid, asteroid_miss));
    }
}

fn bullet_asteroid(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    asteroids: Query<(Entity, &Asteroid)>,
    bullets: Query<(Entity, &Bullet)>,
    mut game_state: ResMut<GameData>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(asteroid_target, bullet_attacker, _) => {
                let (asteroid_ent, _asteroids) = match asteroids.get(*asteroid_target) {
                    Ok(a) => a,
                    Err(_) => return,
                };

                let (bullet_ent, _bullet) = match bullets.get(*bullet_attacker) {
                    Ok(a) => a,
                    Err(_) => return,
                };

                commands.entity(asteroid_ent).despawn();
                game_state.score += 1;
                game_state.asteroids -= 1;
                commands.entity(bullet_ent).despawn();
            }
            _ => {}
        }
    }
}

fn asteroid_miss(
    asteroids: Query<(Entity, &Transform), With<Asteroid>>,
    mut commands: Commands,
    mut game_state: ResMut<GameData>,
) {
    for (entity, transform) in asteroids.iter() {
        if transform.translation.y < -320. {
            commands.entity(entity).despawn();
            game_state.asteroids -= 1;
            game_state.lives -= 1;
        }
    }
}
