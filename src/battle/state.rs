use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BattleState {
    #[default]
    Start,
    BeforeTurn,
    Select,
    Behavior,
    AfterTurn,
    End,
}
