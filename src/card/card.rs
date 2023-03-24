use bevy::prelude::*;

#[derive(Component)]
pub struct Card {
    pub card_id: u32,
    pub name: String,
    pub desc: String,
    pub class: String,
}

#[derive(Component)]
pub enum CardType {
    Character,
    Skill,
    Equip,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Ally;

#[derive(Component)]
pub struct Attack {
    pub value: u32,
}

#[derive(Component)]
pub struct Shield {
    pub value: u32,
}

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component)]
pub struct Heal {
    pub value: u32,
}

#[derive(Component)]
pub struct Speed {
    pub value: u32,
}

#[derive(Component)]
pub struct Cooldown {
    pub value: u32,
}
