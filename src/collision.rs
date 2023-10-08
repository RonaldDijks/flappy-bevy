use bevy::prelude::*;

use crate::{base::Base, bird::Bird, pipe::Pipe, GameState};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(GameState::Playing)));
    }
}

fn update(
    mut next_state: ResMut<NextState<GameState>>,
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>,
    base_query: Query<&Transform, With<Base>>,
) {
    let mut collided = false;
    let bird_transform = match bird_query.get_single() {
        Ok(bird_transform) => bird_transform,
        _ => return,
    };

    for base_transform in base_query.iter() {
        let collision = bevy::sprite::collide_aabb::collide(
            bird_transform.translation,
            Bird::SIZE,
            base_transform.translation,
            Base::SIZE,
        );

        if collision.is_some() {
            collided = true;
        }
    }

    for pipe_transform in pipe_query.iter() {
        let collision = bevy::sprite::collide_aabb::collide(
            bird_transform.translation,
            Bird::SIZE,
            pipe_transform.translation,
            Pipe::SIZE,
        );

        if collision.is_some() {
            collided = true;
        }
    }

    if collided {
        next_state.set(GameState::GameOver);
    }
}
