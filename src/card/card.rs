use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Card {
    pub card_id: u32,
    pub name: String,
    pub desc: String,
}

#[derive(Component, Default)]
pub enum CardType {
    #[default]
    NONE,
    Character,
    Skill,
    Equip,
}

#[derive(Component, Default)]
pub struct Attack(pub u32);

#[derive(Component, Default)]
pub struct Shield(pub u32);

#[derive(Component, Default)]
pub struct Health(pub u32);

#[derive(Component, Default)]
pub struct Heal(pub u32);

#[derive(Component, Default)]
pub struct Speed(pub u32);

#[derive(Component, Default)]
pub struct Cooldown(pub u32);

#[derive(Component, Default)]
pub enum BehaviorType {
    #[default]
    NONE,
    Attack,
    Heal,
    Shield,
    Buff,
    Debuff,
}

#[derive(Component, PartialEq, Default)]
pub enum Class {
    #[default]
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

#[derive(Component, Default)]
pub enum Location {
    #[default]
    NONE,
    Front,
    Back,
}

#[derive(Component, Default)]
pub enum CampType {
    #[default]
    NONE,
    Player,
    Enemy,
    Ally,
}

#[derive(Component, Default)]
pub struct CardSet {
    pub character: CharacterCardBundle,
    pub skill: SkillCardBundle,
    pub equip: EquipCardBundle,
}

#[derive(Bundle, Default)]
pub struct CharacterCardBundle {
    pub card: Card,
    pub attack: Attack,
    pub shield: Shield,
    pub health: Health,
    pub heal: Heal,
    pub speed: Speed,
    pub cooldown: Cooldown,
    pub class: Class,
    pub behavior_type: BehaviorType,
    #[bundle]
    pub sprite: SpriteBundle,
    pub location: Location,
    pub camp_type: CampType,
}

#[derive(Bundle, Default)]
pub struct EquipCardBundle {
    pub card: Card,
    pub attack: Attack,
    pub shield: Shield,
    pub health: Health,
    pub heal: Heal,
    pub speed: Speed,
    pub cooldown: Cooldown,
    pub class: Class,
    pub behavior_type: BehaviorType,
    #[bundle]
    pub sprite: SpriteBundle,
    pub location: Location,
    pub camp_type: CampType,
}

#[derive(Bundle, Default)]
pub struct SkillCardBundle {
    pub card: Card,
    pub attack: Attack,
    pub shield: Shield,
    pub health: Health,
    pub heal: Heal,
    pub speed: Speed,
    pub cooldown: Cooldown,
    pub class: Class,
    pub behavior_type: BehaviorType,
    #[bundle]
    pub sprite: SpriteBundle,
    pub location: Location,
    pub camp_type: CampType,
}
