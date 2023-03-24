use bevy::prelude::*;
use bevy::winit::WinitSettings;

mod card;
mod config;
mod main_menu;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config::GAME_TITLE.to_string(),
                resizable: false,
                resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_state::<states::GameState>()
        .add_startup_system(setup.in_set(OnUpdate(states::GameState::MainMenu)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprite_002.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    },));
    commands.spawn((Button {
        ..Default::default()
    },));
}
