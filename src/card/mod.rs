pub mod card;
pub mod convert;

use bevy::prelude::*;
use serde::*;

use crate::request;
use card::*;

#[derive(Resource)]
struct CardData {
    cards: (),
}

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
        app.add_startup_system(setup).add_system(mouse_button_input);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_info = request::get::<Vec<CardInfo>>("https://secret-dungeon.myroom.dev/card/list");
    let card_vec = convert_json_to_card(card_info, &asset_server);

    let cards = commands.spawn_batch(card_vec);

    commands.insert_resource(CardData { cards });
}

fn convert_json_to_card(
    card_info: Vec<CardInfo>,
    asset_server: &Res<AssetServer>,
) -> Vec<CardBundle> {
    let mut cards = Vec::new();

    for card in card_info {
        cards.push(
            (CardBundle {
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
                    transform: Transform::from_xyz(100.0, 100.0, 0.0),
                    visibility: Visibility::Visible,
                    ..default()
                },
                location: Location::NONE,
                camp_type: CampType::NONE,
                behavior_type: BehaviorType::NONE,
            }),
        );
    }

    cards
}

fn mouse_button_input(buttons: Res<Input<MouseButton>>, &query: Query<&CardBundle>) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("asd");
    }
}
