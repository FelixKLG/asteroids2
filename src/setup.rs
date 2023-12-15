use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (background, camera, colliders));
    }
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("Background.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(800., 800.)),
                ..Default::default()
            },
            texture,
            transform: Transform {
                translation: Vec3::new(0., 0., -5.),
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("Background"),
    ));
}

fn colliders(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(400.0, 5.0))
        .insert(Name::from("Floor collider"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -400.0, 0.0)));

    commands
        .spawn(Collider::cuboid(400.0, 5.0))
        .insert(Name::from("Ceiling collider"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    commands
        .spawn(Collider::cuboid(1., 400.))
        .insert(Name::from("Left collider"))
        .insert(TransformBundle::from(Transform::from_xyz(-400.0, 0.0, 0.0)));

    commands
        .spawn(Collider::cuboid(1., 400.))
        .insert(Name::from("Right collider"))
        .insert(TransformBundle::from(Transform::from_xyz(400.0, 0.0, 0.0)));
}