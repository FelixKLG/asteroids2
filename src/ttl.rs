use bevy::prelude::*;

pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn);
    }
}

#[derive(Component)]
pub struct TimeToLive {
    ttl: Timer,
}

impl TimeToLive {
    pub fn new(time: Timer) -> Self {
        Self { ttl: time }
    }
}

fn despawn(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut TimeToLive)>) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.ttl.tick(time.delta());

        if ttl.ttl.finished() {
            commands.entity(entity).despawn();
        }
    }
}
