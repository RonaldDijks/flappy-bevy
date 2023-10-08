use bevy::prelude::*;

use crate::{bird::BirdEvent, score::ScoreRes, GameState};

const BACKGROUND_COLOR: Color = Color::rgb(234. / 255., 97. / 255., 1. / 255.);
const BORDER_COLOR: Color = Color::rgb(251. / 255., 253. / 255., 235. / 255.);

pub struct UiPlugin;

macro_rules! screen_ui {
    ($app:ident, $bundle:ident, $state:expr) => {
        $app.add_systems(OnEnter($state), $bundle::on_enter)
            .add_systems(Update, $bundle::update.run_if(in_state($state)))
    };
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let app = app.add_systems(Update, ScoreText::update);
        let app = screen_ui!(app, StartingUI, GameState::Starting);
        let app = screen_ui!(app, PlayingUI, GameState::Playing);
        let _app = screen_ui!(app, GameOverUI, GameState::GameOver);
    }
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct ResetButton;

#[derive(Component)]
pub struct ScoreText;

impl ScoreText {
    pub fn new(score: Res<ScoreRes>) -> (ScoreText, TextBundle) {
        (
            ScoreText,
            TextBundle::from_section(
                score.0.to_string(),
                TextStyle {
                    font_size: 38.,
                    ..default()
                },
            ),
        )
    }

    pub fn update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<ScoreRes>) {
        for mut text in query.iter_mut() {
            text.sections[0].value = score.0.to_string();
        }
    }
}

fn despawn(commands: &mut Commands, root_query: Query<Entity, With<UiRoot>>) {
    if let Ok(entity) = root_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Bundle)]
struct StartingUI {
    root: UiRoot,
    sprite: SpriteBundle,
}

impl StartingUI {
    pub fn on_enter(
        mut commands: Commands,
        root_query: Query<Entity, With<UiRoot>>,
        asset_server: Res<AssetServer>,
    ) {
        despawn(&mut commands, root_query);
        commands.spawn(Self {
            root: UiRoot,
            sprite: SpriteBundle {
                texture: asset_server.load("message.png"),
                ..default()
            },
        });
    }

    pub fn update(
        mut next_state: ResMut<NextState<GameState>>,
        keyboard: Res<Input<KeyCode>>,
        mouse: Res<Input<MouseButton>>,
        mut writer: EventWriter<BirdEvent>,
    ) {
        if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
            next_state.set(GameState::Playing);
            writer.send(BirdEvent::Jump);
        }
    }
}

struct PlayingUI;

impl PlayingUI {
    pub fn spawn(mut commands: Commands, score: Res<ScoreRes>) {
        commands
            .spawn((
                UiRoot,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(ScoreText::new(score));
            });
    }

    pub fn on_enter(
        mut commands: Commands,
        score: Res<ScoreRes>,
        root_query: Query<Entity, With<UiRoot>>,
    ) {
        despawn(&mut commands, root_query);
        PlayingUI::spawn(commands, score);
    }

    pub fn update() {}
}

pub struct GameOverUI;

impl GameOverUI {
    pub fn spawn(commands: &mut Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                UiRoot,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        ResetButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(72.0),
                                height: Val::Px(30.0),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(BORDER_COLOR),
                            background_color: BACKGROUND_COLOR.into(),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Restart",
                            TextStyle {
                                font: asset_server.load("Minecraft.ttf"),
                                font_size: 16.0,
                                color: BORDER_COLOR,
                            },
                        ));
                    });
            });
    }

    pub fn on_enter(
        mut commands: Commands,
        root_query: Query<Entity, With<UiRoot>>,
        asset_server: Res<AssetServer>,
    ) {
        despawn(&mut commands, root_query);
        GameOverUI::spawn(&mut commands, asset_server);
    }

    pub fn update(
        keyboard: Res<Input<KeyCode>>,
        mut reset_query: Query<&Interaction, (Changed<Interaction>, With<ResetButton>)>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        if keyboard.just_pressed(KeyCode::Space) {
            next_state.set(GameState::Starting);
        }

        for interaction in &mut reset_query {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::Starting);
            }
        }
    }
}
