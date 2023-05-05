use bevy::prelude::*;

use crate::card::card::*;
use crate::collider::*;

#[derive(Component, Default)]
pub struct DeckEditorInputComponent {
    pub is_dragging: bool,
}

pub struct PcControllerDeckEditorPlugin;

impl Plugin for PcControllerDeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(drag_card)
            .add_system(is_mouse_over);
    }
}

fn setup() {}

fn drag_card(
    mut query: Query<(&mut Hitbox, &mut Transform), With<Card>>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
) {
    if buttons.pressed(MouseButton::Left) {
        for (mut hitbox, mut transform) in query.iter_mut() {
            let cursor_position = windows
                .get_single()
                .expect("can't get window")
                .cursor_position()
                .expect("can't get cursor position");

            if hitbox.on_mouse_over {
                hitbox.origin = cursor_position;
                transform.translation = Vec3::new(
                    cursor_position.x,
                    cursor_position.y,
                    transform.translation.z,
                );
            }
        }
    }
}

fn is_mouse_over(mut query: Query<(&Hitbox, &mut Sprite), With<Card>>) {
    for (hitbox, mut sprite) in query.iter_mut() {
        if hitbox.on_mouse_over {
            sprite.color = Color::rgb(0.5, 0.5, 0.5);
        } else {
            sprite.color = Color::rgb(1., 1., 1.);
        }
    }
}
