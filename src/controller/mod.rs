use bevy::prelude::*;

pub mod pc;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(pc::PcControllerPlugin);
    }
}

fn setup() {}
