use bevy::{
    ecs::event::{Events, ManualEventReader},
    input::mouse::MouseButtonInput,
    prelude::*,
};

#[derive(Component, Default)]
pub struct BattleClickComponent {
    // cursor_position: ManualEventReader<CursorMoved>,
    pub bounds: (Vec2, Vec2),
    pub origin: Vec2,
    pub is_clicked: bool,
    pub pressed_time: f32,
}

pub struct PcControllerBattlePlugin;

impl Plugin for PcControllerBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mouse_over_card)
            .add_system(click_card);
    }
}

fn setup() {}

fn click_card(
    mut query: Query<&mut BattleClickComponent>,
    mut mouse: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
) {
    use bevy::input::ButtonState;
    let window = windows.get_single().unwrap();

    for event in mouse.iter() {
        match event.state {
            ButtonState::Pressed => {}
            ButtonState::Released => {
                for mut clickable in query.iter_mut() {
                    let cursor_position = window.cursor_position().unwrap();

                    if _is_mouse_over(cursor_position, &clickable) {
                        clickable.is_clicked = true;
                    } else {
                    }
                }
            }
        }
    }
}

fn mouse_over_card(
    mut query: Query<(&mut BattleClickComponent, &mut Sprite)>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();

    for (clickable, mut sprite) in query.iter_mut() {
        let cursor_position = match window.cursor_position() {
            Some(position) => position,
            None => return,
        };

        if _is_mouse_over(cursor_position, &clickable) {
            // sprite.color.set_g(0.5);
            sprite.color = Color::rgb(0.5, 0.5, 0.5);
        } else {
            // sprite.color.set_g(1.0);
            sprite.color = Color::rgb(1., 1., 1.);
        }
    }
}

fn _is_mouse_over(mouse_position: Vec2, clickable: &BattleClickComponent) -> bool {
    (clickable.bounds.0.x..clickable.bounds.1.x).contains(&mouse_position.x)
        && (clickable.bounds.0.y..clickable.bounds.1.y).contains(&mouse_position.y)
}
