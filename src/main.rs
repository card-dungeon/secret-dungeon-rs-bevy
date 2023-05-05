use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use dotenv::dotenv;

mod battle;
mod card;
mod collider;
mod config;
mod controller;
mod deck;
mod request;
mod scene_main;
mod states;

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn main() {
    dotenv().ok();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: config::GAME_TITLE.to_string(),
                        resizable: false,
                        resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        // fps 체크
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // 앱이 상호작용을 안하고 있을 경우에 동작 X
        // .insert_resource(WinitSettings::desktop_app())
        .add_state::<states::GameState>()
        .add_startup_system(setup)
        .add_plugin(card::CardPlugin)
        .add_plugin(controller::pc::PcControllerPlugin)
        .add_plugin(deck::DeckPlugin)
        .add_plugin(collider::ColliderPlugin)
        .add_system(go_deck_editor.in_set(OnUpdate(states::GameState::MainMenu)))
        .add_system(delete_button.in_schedule(OnExit(states::GameState::MainMenu)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(360., 540., 1.)),
        ..default()
    });

    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/DungGeunMo.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn go_deck_editor(
    mut next_state: ResMut<NextState<states::GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // *color = PRESSED_BUTTON.into();
                next_state.set(states::GameState::DeckEditor);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn delete_button(mut commands: Commands, menu_data: ResMut<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

// fn text_input(
//     mut char_evr: EventReader<ReceivedCharacter>,
//     keys: Res<Input<KeyCode>>,
//     mut string: Local<String>,
// ) {
//     for ev in char_evr.iter() {
//         println!("Got char: '{}'", ev.char);
//         string.push(ev.char);
//     }

//     if keys.just_pressed(KeyCode::Return) {
//         println!("Text input: {}", *string);
//         string.clear();
//     }
// }
