use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SystemSet)]
pub enum BattleSystemSet {
    #[default]
    Start,
    BeforeTurn,
    Select,
    Behavior,
    AfterTurn,
    End,
}
