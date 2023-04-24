use bevy::prelude::*;

use crate::card::card::*;
use crate::deck::manager::*;

#[derive(Component)]
pub struct CardPanel {
    pub character_card: CharacterCardBundle,
    pub skill_card: SkillCardBundle,
    pub equip_card: EquipCardBundle,
    pub location: Location,
}

#[derive(Component)]
pub enum Location {
    None,
    Front,
    Back,
}

fn deck_building(mut query: Query<(&CardPanel)>) {}
