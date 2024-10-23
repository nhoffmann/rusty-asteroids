use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::ship::Ship;
use crate::{GameState, Heading, Position};

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .init_resource::<FiredAction>()
            .add_systems(
                Update,
                (set_movement_actions, set_fired_actions).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

#[derive(Default, Resource, Debug)]
pub struct FiredAction {
    pub heading: Option<Heading>,
    pub position: Option<Position>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
}

pub fn set_fired_actions(
    mut actions: ResMut<FiredAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ship_query: Query<&Transform, With<Ship>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let ship_transform = ship_query.single();
        actions.heading = Some(Heading(ship_transform.rotation * Vec3::Y));
        actions.position = Some(Position(ship_transform.translation.truncate()));
    } else {
        actions.heading = None;
        actions.position = None;
    }
}
