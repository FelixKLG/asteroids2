use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyPosition};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, move_pos);
    }
}

#[derive(Component)]
struct Player;

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("Rocket.png");

    commands
        .spawn(Player)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(25., 50.))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: false,
            ..Default::default()
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 100.)),
                ..Default::default()
            },
            texture,
            transform: Transform {
                translation: Vec3::new(0., -350., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::from("Player"));
}

fn move_pos(
    mut characters: Query<(&mut Transform, &Player, With<RigidBody>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let movement_speed = 100.0 * time.delta_seconds();

    for (mut transform, _, _) in characters.iter_mut() {
        let translation = &mut transform.translation.x;

        if input.pressed(KeyCode::Left) {
            *translation -= movement_speed;
        }
        if input.pressed(KeyCode::Right) {
            *translation += movement_speed;
        }
    }
}
