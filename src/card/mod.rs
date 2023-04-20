pub mod battle;
pub mod card;
pub mod convert;

use bevy::prelude::*;
use serde::*;

use crate::controller::pc::battle::*;
use crate::request;
use card::*;

const CARD_WIDTH: f32 = 128.0;
const CARD_HEIGHT: f32 = 200.0;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardInfo {
    pub card_id: u32,
    pub name: String,
    pub desc: String,
    pub sprite: String,
    pub atk: u32,
    pub hp: u32,
    pub sd: u32,
    pub spd: u32,
    pub heal: u32,
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(toggle_card_set);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_info = request::get::<Vec<CardInfo>>(
        std::env::var("API_CARD_LIST")
            .expect("NOT SET API URL")
            .as_str(),
    );
    let mut card_vec = convert_json_to_card(card_info, &asset_server);

    // let cards = commands.spawn_batch(card_vec);

    // for card in card_vec {
    commands
        .spawn(card_vec.pop().unwrap())
        .insert(BattleClickComponent {
            bounds: (
                Vec2::new(500. - (CARD_WIDTH / 2.), 500. - (CARD_HEIGHT / 2.)),
                Vec2::new(500. + (CARD_WIDTH / 2.), 500. + (CARD_HEIGHT / 2.)),
            ),
            origin: Vec2::new(500., 500.),
            ..Default::default()
        });
    commands
        .spawn(card_vec.pop().unwrap())
        .insert(BattleClickComponent {
            bounds: (
                Vec2::new(300. - (CARD_WIDTH / 2.), 300. - (CARD_HEIGHT / 2.)),
                Vec2::new(300. + (CARD_WIDTH / 2.), 300. + (CARD_HEIGHT / 2.)),
            ),
            origin: Vec2::new(300., 300.),
            ..Default::default()
        });
    // }

    // commands.insert_resource(CardData { cards });
}

fn convert_json_to_card(
    card_info: Vec<CardInfo>,
    asset_server: &Res<AssetServer>,
) -> Vec<CardBundle> {
    let mut cards = Vec::new();

    for card in card_info {
        cards.push(CardBundle {
            card: Card {
                card_id: card.card_id,
                name: card.name,
                desc: card.desc,
            },
            card_type: CardType::Character,
            attack: Attack(card.atk),
            shield: Shield(card.sd),
            health: Health(card.hp),
            heal: Heal(card.heal),
            speed: Speed(card.spd),
            cooldown: Cooldown(1),
            class: Class::NONE,
            sprite: SpriteBundle {
                texture: asset_server.load(card.sprite),
                transform: Transform::from_translation(Vec3::new(300., 300., 1.)),
                visibility: Visibility::Visible,
                ..default()
            },
            location: Location::NONE,
            camp_type: CampType::NONE,
            behavior_type: BehaviorType::NONE,
        });
    }
    cards.push(CardBundle {
        card: Card {
            card_id: 0,
            name: "test".to_string(),
            desc: "test".to_string(),
        },
        card_type: CardType::Character,
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
    cards
}
