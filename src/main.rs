use base::BasePlugin;
use bevy::{app::AppExit, prelude::*, window::WindowResolution};
use bird::BirdPlugin;
use collision::CollisionPlugin;
use pipe::PipePlugin;
use score::ScorePlugin;
use ui::UiPlugin;

mod animation;
mod base;
mod bird;
mod collision;
mod pipe;
mod score;
mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Starting,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(288.0, 440.0),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<GameState>()
        .add_plugins(BasePlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(PipePlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_exit_keypress)
        .run();
}

fn handle_exit_keypress(input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("background-day.png");
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture,
        ..default()
    });
}
