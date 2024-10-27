use bevy::prelude::*;

use crate::{
    asteroids::{Asteroid, AsteroidSize},
    ship::Ship,
    GameState, Hit,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                FixedUpdate,
                (handle_player_hit, handle_asteroid_hit).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Menu), despawn_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub life_count: u8,
    pub score: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            life_count: 3,
            score: 0,
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player::new());
}

fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn_recursive();
    }
}

fn handle_player_hit(
    mut player_query: Query<&mut Player>,
    ship_query: Query<&Hit, With<Ship>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for _ in &ship_query {
            player.life_count -= 1;

            if player.life_count == 0 {
                next_state.set(GameState::Menu);
            }
        }
    }
}

fn handle_asteroid_hit(
    mut player_query: Query<&mut Player>,
    asteroid_query: Query<(&AsteroidSize, &Hit), With<Asteroid>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for (size, _) in &asteroid_query {
            player.score += match size {
                AsteroidSize::Large => 20,
                AsteroidSize::Medium => 50,
                AsteroidSize::Small => 100,
            };
        }
    }
}
