use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CardHitbox {
    pub bounds: (Vec2, Vec2),
    pub origin: Vec2,
    pub width: f32,
    pub height: f32,
}

pub struct CardColliderPlugin;

impl Plugin for CardColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(track_origin);
    }
}

fn setup() {}

fn track_origin(mut query: Query<&mut CardHitbox>) {
    for mut hitbox in query.iter_mut() {
        hitbox.bounds.0.x = hitbox.origin.x - (hitbox.width / 2.);
        hitbox.bounds.0.y = hitbox.origin.y - (hitbox.height / 2.);
        hitbox.bounds.1.x = hitbox.origin.x + (hitbox.width / 2.);
        hitbox.bounds.1.y = hitbox.origin.y + (hitbox.height / 2.);
        info!("{:?}", hitbox.bounds);
    }
}
