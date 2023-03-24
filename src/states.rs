use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    DeckEditor,
    Shop,
    Battle,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BattleState {
    #[default]
    Start,
    PlayerTurn,
    EnemyTurn,
    End,
}
