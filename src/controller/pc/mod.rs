use bevy::prelude::*;

use crate::collider::Hitbox;

pub mod battle;
pub mod deck_editor;

#[derive(Resource)]
pub struct Mouse {
    pub click_timer: f32,
    pub is_click: bool,
    pub is_drag: bool,
    pub is_drop: bool,
    pub entity: Option<Entity>,
}

pub struct PcControllerPlugin;

impl Plugin for PcControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Mouse {
                click_timer: 0.,
                is_click: false,
                is_drag: false,
                is_drop: false,
                entity: None,
            })
            .add_plugin(battle::PcControllerBattlePlugin)
            .add_plugin(deck_editor::PcControllerDeckEditorPlugin)
            .add_system(click_timer);
    }
}

fn setup() {}

fn click_timer(
    mut query: Query<(Entity, &Hitbox)>,
    mut mouse: ResMut<Mouse>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    if buttons.pressed(MouseButton::Left) {
        for (entity, hitbox) in query.iter_mut() {
            if hitbox.on_mouse_over {
                mouse.click_timer += time.delta_seconds();

                if mouse.click_timer > 0.25 {
                    mouse.is_drag = true;
                    mouse.entity = Some(entity);
                    break;
                }
            }
        }
    }

    if buttons.just_released(MouseButton::Left) {
        for (entity, hitbox) in query.iter_mut() {
            if hitbox.on_mouse_over {
                if mouse.click_timer < 0.25 {
                    mouse.is_click = true;
                    mouse.click_timer = 0.;
                    break;
                }

                if mouse.is_drag {
                    mouse.is_drop = true;
                    mouse.is_drag = false;
                    mouse.click_timer = 0.;
                    break;
                }
            }
        }
    }
}
