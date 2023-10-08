use bevy::{audio::VolumeLevel, prelude::*};

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreRes>()
            .add_event::<ScoreEvent>()
            .add_systems(OnEnter(GameState::Starting), reset)
            .add_systems(Update, on_event);
    }
}

#[derive(Resource, Default)]
pub struct ScoreRes(pub u32);

#[derive(Event)]
pub struct ScoreEvent;

fn reset(mut score: ResMut<ScoreRes>) {
    score.0 = 0;
}

fn on_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<ScoreRes>,
    mut reader: EventReader<ScoreEvent>,
) {
    if !reader.is_empty() {
        reader.clear();
        score.0 += 1;

        commands.spawn(AudioBundle {
            source: asset_server.load("point.ogg"),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.5)),
                ..default()
            },
            ..default()
        });
    }
}
