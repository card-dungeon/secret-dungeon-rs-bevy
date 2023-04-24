pub mod editor;
pub mod manager;

use bevy::prelude::*;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {}
