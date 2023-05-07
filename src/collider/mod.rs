use bevy::prelude::*;

#[derive(Component, Debug)]
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

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(track_origin)
            .add_system(is_mouse_over)
            .add_system(track_transform)
            .add_system(is_hit);
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

fn track_transform(mut query: Query<(&mut Hitbox, &Transform)>) {
    for (mut hitbox, transform) in query.iter_mut() {
        hitbox.origin.x = transform.translation.x;
        hitbox.origin.y = transform.translation.y;
    }
}

fn is_mouse_over(mut query: Query<&mut Hitbox>, windows: Query<&Window>) {
    for mut hitbox in query.iter_mut() {
        let window = windows.get_single().expect("can't get window");
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

fn is_hit(query: Query<&Hitbox>) {
    let mut tmp_object: &Hitbox;
    for hitbox in query.iter() {
        tmp_object = hitbox;
        // info!("{:?}", hitbox2);
        // if _is_hit_object_to_object(&hitbox, &hitbox2) {
        //     println!("hit");
        // }
    }
}

fn _is_mouse_over(mouse_position: &Vec2, hitbox: &Hitbox) -> bool {
    (hitbox.bounds.0.x..hitbox.bounds.1.x).contains(&mouse_position.x)
        && (hitbox.bounds.0.y..hitbox.bounds.1.y).contains(&mouse_position.y)
}

fn _is_hit_object_to_object(box1: &Hitbox, box2: &Hitbox) -> bool {
    (box1.bounds.0.x..box1.bounds.1.x).contains(&box2.bounds.0.x)
        && (box1.bounds.0.y..box1.bounds.1.y).contains(&box2.bounds.0.y)
        && (box1.bounds.0.x..box1.bounds.1.x).contains(&box2.bounds.1.x)
        && (box1.bounds.0.y..box1.bounds.1.y).contains(&box2.bounds.1.y)
}
