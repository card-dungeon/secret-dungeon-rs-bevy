use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::card::card::*;
use crate::collider::*;

#[derive(Component, Default)]
pub struct DeckEditorInputComponent {
    pub hitbox: CardHitbox,
    pub is_dragging: bool,
}

pub struct PcControllerDeckEditorPlugin;

impl Plugin for PcControllerDeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(drag_card);
    }
}

fn setup() {}

fn drag_card(
    mut query: Query<(&mut Card, &mut DeckEditorInputComponent, &mut Transform)>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
) {
    use bevy::input::ButtonState;
    let window = windows.get_single().unwrap();

    if buttons.pressed(MouseButton::Left) {
        for (mut _card, mut interactable, mut transform) in query.iter_mut() {
            let cursor_position = window.cursor_position().unwrap();

            if _is_mouse_over(&cursor_position, &interactable) {
                interactable.is_dragging = true;
                interactable.hitbox.origin = cursor_position;
                transform.translation = Vec3::new(
                    cursor_position.x,
                    cursor_position.y,
                    transform.translation.z,
                );
            }
        }
    }
}

fn _is_mouse_over(mouse_position: &Vec2, interactable: &DeckEditorInputComponent) -> bool {
    (interactable.hitbox.bounds.0.x..interactable.hitbox.bounds.1.x).contains(&mouse_position.x)
        && (interactable.hitbox.bounds.0.y..interactable.hitbox.bounds.1.y)
            .contains(&mouse_position.y)
}
