pub mod battle;
pub mod card;
pub mod convert;

use bevy::prelude::*;
use serde::*;

use crate::collider::*;
use crate::controller::pc::battle::*;
use crate::controller::pc::deck_editor::*;
use crate::request;
use card::*;

use self::convert::convert_json_to_card;

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
        app.add_startup_system(setup);
        // .add_system(toggle_card_set);
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
    let card_1 = card_vec.pop().unwrap();
    commands
        .spawn(card_1)
        .insert(Hitbox {
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            ..Default::default()
        })
        .insert(BattleInputComponent {
            ..Default::default()
        })
        .insert(DeckEditorInputComponent {
            ..Default::default()
        });

    let card_2 = card_vec.pop().unwrap();
    commands
        .spawn(card_2)
        .insert(Hitbox {
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            ..Default::default()
        })
        .insert(BattleInputComponent {
            ..Default::default()
        })
        .insert(DeckEditorInputComponent {
            ..Default::default()
        });

    // commands.insert_resource(CardData { cards });
}
