use bevy::prelude::*;

use crate::GameState;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BaseSpeed(100.))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Starting), on_enter_starting)
            .add_systems(OnEnter(GameState::Playing), on_enter_playing)
            .add_systems(Update, update.run_if(not(in_state(GameState::GameOver))));
    }
}

#[derive(Resource)]
pub struct BaseSpeed(f32);

#[derive(Component, Default)]
pub struct Base;

impl Base {
    pub const SIZE: Vec2 = Vec2::new(336., 112.);

    pub fn bundle(asset_server: &Res<AssetServer>, position: Vec2) -> (Self, SpriteBundle) {
        (
            Base,
            SpriteBundle {
                transform: Transform {
                    translation: position.extend(1.0),
                    ..Default::default()
                },
                texture: asset_server.load("base.png"),
                ..Default::default()
            },
        )
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let left_position = Vec2::new(0.0, -200.0);
    let right_position = Vec2::new(0. + Base::SIZE.x, -200.);

    commands.spawn(Base::bundle(&asset_server, left_position));
    commands.spawn(Base::bundle(&asset_server, right_position));
}

fn on_enter_starting(mut speed: ResMut<BaseSpeed>) {
    speed.0 = 150.0;
}

fn on_enter_playing(mut speed: ResMut<BaseSpeed>) {
    speed.0 = 100.0;
}

fn update(speed: Res<BaseSpeed>, mut query: Query<&mut Transform, With<Base>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= speed.0 * time.delta_seconds();
        if transform.translation.x < -Base::SIZE.x {
            transform.translation.x += Base::SIZE.x * 2.;
        }
    }
}
