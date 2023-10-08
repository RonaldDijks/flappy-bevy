use std::time::Duration;

use bevy::prelude::*;

use crate::GameState;

struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(GameState::Playing)));
    }
}

fn update(mut query: Query<(&mut Animation, &mut Handle<Image>)>, time: Res<Time>) {
    for (mut animation, mut texture) in query.iter_mut() {
        animation.tick(time.delta());
        if animation.just_finished() {
            *texture = animation.current().clone();
        }
    }
}

#[derive(Component)]
pub struct Animation {
    timer: Timer,
    frames: Vec<Handle<Image>>,
    current_frame: usize,
}

impl Animation {
    pub fn new(frames: Vec<Handle<Image>>) -> Animation {
        Animation {
            frames,
            current_frame: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }

    pub fn current(&self) -> &Handle<Image> {
        &self.frames[self.current_frame]
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if self.timer.just_finished() {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }
}
