use bevy::prelude::*;

#[derive(Component)]
pub struct Card {
    pub card_id: u32,
    pub name: String,
    pub desc: String,
}

#[derive(Component)]
pub enum CardType {
    Character,
    Skill,
    Equip,
}

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Shield(pub u32);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Heal(pub u32);

#[derive(Component)]
pub struct Speed(pub u32);

#[derive(Component)]
pub struct Cooldown(pub u32);

#[derive(Component, PartialEq)]
pub enum Class {
    NONE,
    Warrior,
    Mage,
    Priest,
    Rogue,
    Hunter,
    Druid,
    Shaman,
    Paladin,
    Warlock,
}

#[derive(Component)]
pub enum Location {
    NONE,
    Front,
    Back,
}

#[derive(Component)]
pub struct Back;

#[derive(Component)]
pub enum CampType {
    NONE,
    Player,
    Enemy,
    Ally,
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub card_type: CardType,
    pub attack: Attack,
    pub shield: Shield,
    pub health: Health,
    pub heal: Heal,
    pub speed: Speed,
    pub cooldown: Cooldown,
    pub class: Class,
    pub sprite: SpriteBundle,
    pub location: Location,
    pub camp_type: CampType,
}
