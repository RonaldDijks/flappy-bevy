use bevy::{audio::VolumeLevel, prelude::*};

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreRes>()
            .init_resource::<ScoreSound>()
            .add_event::<ScoreEvent>()
            .add_systems(Startup, init)
            .add_systems(OnEnter(GameState::Starting), reset)
            .add_systems(Update, on_event);
    }
}

#[derive(Resource, Default)]
pub struct ScoreSound(Handle<AudioSource>);

#[derive(Resource, Default)]
pub struct ScoreRes(pub u32);

#[derive(Event)]
pub struct ScoreEvent;

fn init(asset_server: Res<AssetServer>, mut score_sound: ResMut<ScoreSound>) {
    let sound = asset_server.load("point.ogg");
    score_sound.0 = sound;
}

fn reset(mut score: ResMut<ScoreRes>) {
    score.0 = 0;
}

fn on_event(
    mut commands: Commands,
    mut score: ResMut<ScoreRes>,
    mut reader: EventReader<ScoreEvent>,
    score_sound: Res<ScoreSound>,
) {
    if !reader.is_empty() {
        reader.clear();
        score.0 += 1;

        commands.spawn(AudioBundle {
            source: score_sound.0.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.5)),
                ..default()
            },
            ..default()
        });
    }
}
