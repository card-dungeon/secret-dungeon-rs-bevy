pub mod card;

use bevy::prelude::*;

use card::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(clicked);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Card {
            card_id: 1,
            name: "Test Card".to_string(),
            desc: "This is a test card.".to_string(),
            class: "Test".to_string(),
        },
        CardType::Character,
        Player,
        Attack { value: 1 },
        Shield { value: 1 },
        Health { value: 1 },
        Heal { value: 1 },
        Speed { value: 1 },
        Cooldown { value: 1 },
        SpriteBundle {
            texture: asset_server.load("sprite_002.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
}

fn clicked(mut query: Query<Entity, With<Card>>, mouse: Res<Input<MouseButton>>) {
    if mouse.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
    // for (interaction, entity) in query.iter_mut() {
    //     println!("Clicked!");
    //     match *interaction {
    //         Interaction::Clicked => {
    //             // Get the entity that was clicked
    //             println!("Button clicked: {:?}", entity);

    //             // Update the text of the clicked button
    //             // if let Ok(mut text) = text_query.get_mut(entity) {
    //             //     text.sections[0].value = "Clicked!".to_string();
    //             // }
    //         }
    //         _ => {}
    //     }
    // }
}
