use bevy::prelude::*;

use crate::card::card::*;
use crate::deck::editor::*;

const DECK_ID: u32 = 1;

#[derive(Component, Default)]
pub struct Deck {
    pub deck_id: u32,
    pub name: String,
    pub desc: String,
    pub card_panel: Vec<CardPanel>,
}
