use bevy::prelude::*;

pub mod after_battle;
pub mod after_turn;
pub mod before_battle;
pub mod before_turn;
pub mod behavior;
pub mod select;
pub mod state;
pub mod system_set;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<state::BattleState>()
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {}
