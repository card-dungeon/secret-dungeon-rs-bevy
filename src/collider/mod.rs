use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    pub bounds: (Vec2, Vec2),
    pub origin: Vec2,
    pub width: f32,
    pub height: f32,
    pub on_mouse_over: bool,
}

impl Default for Hitbox {
    fn default() -> Self {
        Self {
            bounds: (Vec2::new(0., 0.), Vec2::new(0., 0.)),
            origin: Vec2::new(-100., -100.),
            width: 0.,
            height: 0.,
            on_mouse_over: false,
        }
    }
}

pub struct CardColliderPlugin;

impl Plugin for CardColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(track_origin)
            .add_system(is_mouse_over);
    }
}

fn setup() {}

fn track_origin(mut query: Query<&mut Hitbox>) {
    for mut hitbox in query.iter_mut() {
        hitbox.bounds.0.x = hitbox.origin.x - (hitbox.width / 2.);
        hitbox.bounds.0.y = hitbox.origin.y - (hitbox.height / 2.);
        hitbox.bounds.1.x = hitbox.origin.x + (hitbox.width / 2.);
        hitbox.bounds.1.y = hitbox.origin.y + (hitbox.height / 2.);
    }
}

fn is_mouse_over(mut query: Query<&mut Hitbox>, windows: Query<&Window>) {
    let window = windows.get_single().unwrap();

    for mut hitbox in query.iter_mut() {
        let cursor_position = match window.cursor_position() {
            Some(position) => position,
            None => return,
        };

        if _is_mouse_over(&cursor_position, &hitbox) {
            hitbox.on_mouse_over = true;
        } else {
            hitbox.on_mouse_over = false;
        }
    }
}

fn _is_mouse_over(mouse_position: &Vec2, hitbox: &Hitbox) -> bool {
    (hitbox.bounds.0.x..hitbox.bounds.1.x).contains(&mouse_position.x)
        && (hitbox.bounds.0.y..hitbox.bounds.1.y).contains(&mouse_position.y)
}
