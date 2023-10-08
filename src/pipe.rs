use bevy::prelude::*;

use crate::{bird::Bird, score::ScoreEvent, GameState};

const SPEED: f32 = 100.0;
const PIPE_SIZE: Vec2 = Vec2::new(52.0, 320.0);
const BACKGROUND_SIZE: Vec2 = Vec2::new(288.0, 512.0);

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Starting), spawn)
            .add_systems(OnExit(GameState::GameOver), despawn.after(spawn))
            .add_systems(
                Update,
                (update_timer, update_position).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct PipeVelocity(Vec2);

#[derive(Component)]
pub struct Pipe;

impl Pipe {
    pub const SIZE: Vec2 = Vec2::new(52.0, 320.0);
}

#[derive(Component)]
struct PipeTimer(Timer);

fn spawn(mut commands: Commands) {
    commands.spawn(PipeTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

fn despawn(
    mut commands: Commands,
    timer_query: Query<Entity, With<PipeTimer>>,
    pipe_query: Query<Entity, With<Pipe>>,
) {
    for timer_entity in timer_query.iter() {
        commands.entity(timer_entity).despawn_recursive();
    }

    for pipe_entity in pipe_query.iter() {
        commands.entity(pipe_entity).despawn_recursive();
    }
}

fn update_timer(
    mut commands: Commands,
    mut timer_query: Query<&mut PipeTimer>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    const INITIAL_OFFSET: f32 = 300.;
    const GAP: f32 = 60.;
    const OPENING: f32 = 0.;

    for mut timer in timer_query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.spawn((
                Pipe,
                PipeVelocity(Vec2::new(-SPEED, 0.0)),
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            INITIAL_OFFSET,
                            (-PIPE_SIZE.y / 2.) - GAP + OPENING,
                            0f32,
                        ),
                        ..Default::default()
                    },
                    texture: asset_server.load("pipe-green.png"),
                    ..Default::default()
                },
            ));

            commands.spawn((
                Pipe,
                PipeVelocity(Vec2::new(-SPEED, 0.0)),
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            INITIAL_OFFSET,
                            (PIPE_SIZE.y / 2.) + GAP + OPENING,
                            0f32,
                        ),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        flip_y: true,
                        ..Default::default()
                    },
                    texture: asset_server.load("pipe-green.png"),
                    ..Default::default()
                },
            ));
        }
    }
}

fn update_position(
    mut commands: Commands,
    mut query: Query<(Entity, &PipeVelocity, &mut Transform)>,
    time: Res<Time>,
    mut writer: EventWriter<ScoreEvent>,
) {
    let background_rect = Rect::from_center_size(Vec2::ZERO, BACKGROUND_SIZE);
    for (entity, velocity, mut transform) in query.iter_mut() {
        let old_translation = transform.translation;
        let old_rect = Rect::from_center_size(old_translation.truncate(), Pipe::SIZE);
        let new_translation = old_translation + velocity.0.extend(0.) * time.delta_seconds();
        let new_rect = Rect::from_center_size(new_translation.truncate(), Pipe::SIZE);
        transform.translation = new_translation;

        if old_rect.center().x >= Bird::X && new_rect.center().x <= Bird::X {
            writer.send(ScoreEvent);
        }

        if new_rect.max.x < background_rect.min.x {
            commands.entity(entity).despawn_recursive();
        }
    }
}
