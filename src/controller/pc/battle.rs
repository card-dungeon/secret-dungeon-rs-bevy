use bevy::{
    ecs::event::{Events, ManualEventReader},
    prelude::*,
};

use crate::card::card::BoundBox;

#[derive(Component)]
pub struct BattleClickComponent {
    // cursor_position: ManualEventReader<CursorMoved>,
    pub bounds: (Vec2, Vec2),
    pub origin: Vec2,
}

pub struct PcControllerBattlePlugin;

impl Plugin for PcControllerBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(mouse_over_card);
        // .add_system(cursor_events);
    }
}

fn setup() {}

fn click(mut query: Query<(&Window, &mut BoundBox)>, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        for (_window, mut interact_entity) in &mut query {
            info!("Mouse button pressed at: ");
        }
    }
}

fn mouse_over_card(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<(Entity, &BattleClickComponent, &mut Sprite)>,
) {
    for (entity, clickable, mut sprite) in query.iter_mut() {
        for event in cursor.iter() {
            if (clickable.bounds.0.x..clickable.bounds.1.x).contains(&event.position.x)
                && (clickable.bounds.0.y..clickable.bounds.1.y).contains(&event.position.y)
            {
                sprite.color.set_g(0.5);
            } else {
                sprite.color.set_g(1.0);
            }
        }
    }
}
