use bevy::prelude::*;

use crate::{player::Player, GameState, TEXT_COLOR, TEXT_SIZE};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_lifes_ui, spawn_score_ui),
        )
        .add_systems(
            Update,
            (update_score_ui, update_lifes_ui).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Menu), despawn_ui);
    }
}

#[derive(Component)]
pub struct LifesUI;

#[derive(Component)]
pub struct ScoreUI;

pub fn spawn_lifes_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: TEXT_SIZE,
            color: TEXT_COLOR,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.),
            left: Val::Px(0.),
            ..default()
        }),
        LifesUI,
    ));
}

pub fn spawn_score_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: TEXT_SIZE,
            color: TEXT_COLOR,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            left: Val::Px(0.),
            ..default()
        }),
        ScoreUI,
    ));
}

fn despawn_ui(
    mut commands: Commands,
    score_ui_query: Query<Entity, With<ScoreUI>>,
    lifes_ui_query: Query<Entity, With<LifesUI>>,
) {
    score_ui_query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
    lifes_ui_query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
}

pub fn update_lifes_ui(
    player: Query<&Player>,
    mut lifes_ui_query: Query<&mut Text, With<LifesUI>>,
) {
    if let Ok(player) = player.get_single() {
        if let Ok(mut text) = lifes_ui_query.get_single_mut() {
            text.sections[0].value = player.life_count.to_string();
        }
    }
}

pub fn update_score_ui(
    player: Query<&Player>,
    mut score_ui_query: Query<&mut Text, With<ScoreUI>>,
) {
    if let Ok(player) = player.get_single() {
        if let Ok(mut text) = score_ui_query.get_single_mut() {
            text.sections[0].value = player.score.to_string();
        }
    }
}
