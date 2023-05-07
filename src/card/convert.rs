use crate::card::*;

pub fn string_to_class(class: &str) -> Class {
    match class {
        "WARRIOR" => Class::Warrior,
        "MAGE" => Class::Mage,
        "PRIEST" => Class::Priest,
        "ROGUE" => Class::Rogue,
        "HUNTER" => Class::Hunter,
        "DRUID" => Class::Druid,
        "SHAMAN" => Class::Shaman,
        "PALADIN" => Class::Paladin,
        "WARLOCK" => Class::Warlock,
        _ => Class::NONE,
    }
}

pub fn convert_json_to_card(
    card_info: Vec<CardInfo>,
    asset_server: &Res<AssetServer>,
) -> Vec<CharacterCardBundle> {
    let mut cards = Vec::new();

    // for card in card_info {
    //     cards.push(CharacterCardBundle {
    //         card: Card {
    //             card_id: card.card_id,
    //             name: card.name,
    //             desc: card.desc,
    //         },
    //         // card_type: CardType::Character,
    //         attack: Attack(card.atk),
    //         shield: Shield(card.sd),
    //         health: Health(card.hp),
    //         heal: Heal(card.heal),
    //         speed: Speed(card.spd),
    //         cooldown: Cooldown(1),
    //         class: Class::NONE,
    //         sprite: SpriteBundle {
    //             texture: asset_server.load(card.sprite),
    //             transform: Transform::from_translation(Vec3::new(300., 300., 1.)),
    //             visibility: Visibility::Visible,
    //             ..default()
    //         },
    //         location: Location::NONE,
    //         camp_type: CampType::NONE,
    //         behavior_type: BehaviorType::NONE,
    //     });
    // }
    cards.push(CharacterCardBundle {
        card: Card {
            card_id: 0,
            name: "test1".to_string(),
            desc: "test1".to_string(),
        },
        // card_type: CardType::Character,
        attack: Attack(1),
        shield: Shield(1),
        health: Health(1),
        heal: Heal(1),
        speed: Speed(1),
        cooldown: Cooldown(1),
        class: Class::NONE,
        sprite: SpriteBundle {
            texture: asset_server.load("warrior_002.png"),
            transform: Transform::from_translation(Vec3::new(500., 500., 1.)),
            visibility: Visibility::Visible,
            ..default()
        },
        location: Location::NONE,
        camp_type: CampType::NONE,
        behavior_type: BehaviorType::NONE,
    });
    cards.push(CharacterCardBundle {
        card: Card {
            card_id: 0,
            name: "test2".to_string(),
            desc: "test2".to_string(),
        },
        // card_type: CardType::Character,
        attack: Attack(1),
        shield: Shield(1),
        health: Health(1),
        heal: Heal(1),
        speed: Speed(1),
        cooldown: Cooldown(1),
        class: Class::NONE,
        sprite: SpriteBundle {
            texture: asset_server.load("warrior_002.png"),
            transform: Transform::from_translation(Vec3::new(300., 300., 1.)),
            visibility: Visibility::Visible,
            ..default()
        },
        location: Location::NONE,
        camp_type: CampType::NONE,
        behavior_type: BehaviorType::NONE,
    });

    cards
}
