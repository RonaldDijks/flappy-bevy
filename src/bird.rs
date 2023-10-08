use std::path::Path;

use bevy::prelude::*;

const DEFAULT_VELOCITY: f32 = 2.5;

use crate::{animation::Animation, GameState};
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BirdEvent>()
            .add_systems(OnEnter(GameState::Starting), spawn)
            .add_systems(OnExit(GameState::GameOver), despawn.before(spawn))
            .add_systems(Update, bobble.run_if(in_state(GameState::Starting)))
            .add_systems(
                Update,
                (update, handle_input).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Bobble(f32);

#[derive(Component)]
pub struct Bird {
    velocity: f32,
    rotation: f32,
}

#[derive(Event)]
pub enum BirdEvent {
    Jump,
}

impl Bird {
    pub const X: f32 = -60.;
    pub const SIZE: Vec2 = Vec2::new(32.0, 24.0);
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let animation = Animation::new(vec![
        asset_server.load(Path::new("yellowbird-midflap.png")),
        asset_server.load("yellowbird-upflap.png"),
        asset_server.load("yellowbird-downflap.png"),
    ]);

    commands.spawn((
        Bird {
            velocity: 0.,
            rotation: 0.,
        },
        SpriteBundle {
            texture: animation.current().clone(),
            transform: Transform::from_xyz(Bird::X, 0.0, 0.0),
            ..Default::default()
        },
        animation,
        Bobble(0.),
    ));
}

fn despawn(mut commands: Commands, query: Query<Entity, With<Bird>>) {
    for bird_entity in query.iter() {
        commands.entity(bird_entity).despawn();
    }
}

fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Bird, &mut Transform)>,
    mut reader: EventReader<BirdEvent>,
) {
    const GRAVITY: f32 = 0.098;
    const ROTATION_DELTA: f32 = 1.5;

    let (mut player, mut transform) = query.single_mut();

    player.velocity -= GRAVITY;
    player.rotation -= ROTATION_DELTA;

    if !reader.is_empty() {
        reader.clear();

        player.velocity = DEFAULT_VELOCITY;
        player.rotation = 50.;

        commands.spawn(AudioBundle {
            source: asset_server.load("wing.ogg"),
            ..default()
        });
    }

    transform.translation.y += player.velocity;
    transform.rotation = Quat::from_rotation_z(player.rotation.clamp(-90., 25.).to_radians());
}

fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut writer: EventWriter<BirdEvent>,
) {
    if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        writer.send(BirdEvent::Jump);
    }
}

fn bobble(mut query: Query<(&mut Bobble, &mut Transform)>) {
    for (mut bobble, mut transform) in query.iter_mut() {
        let y = bobble.0.sin() * 20.;
        transform.translation.y = y;

        bobble.0 += std::f32::consts::PI / 100.;
    }
}
