use bevy::prelude::*;

pub mod battle;
pub mod deck_editor;

pub struct PcControllerPlugin;

impl Plugin for PcControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(battle::PcControllerBattlePlugin);
    }
}

fn setup() {}
