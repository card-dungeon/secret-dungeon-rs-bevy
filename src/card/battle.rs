use bevy::prelude::*;

use crate::controller::pc::battle::BattleInputComponent;

pub fn toggle_card_set(mut query: Query<&BattleInputComponent>) {
    for card in query.iter_mut() {
        if card.is_clicked {
            println!("toggle card set");
        }
    }
}
