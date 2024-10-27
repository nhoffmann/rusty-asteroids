use bevy::prelude::*;

use crate::{GameState, TEXT_COLOR};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(
                Update,
                (handle_menu_buttons).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), despawn_menu);
    }
}

#[derive(Component, Debug)]
struct Menu;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum ButtonAction {
    StartGame,
}

fn spawn_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                ..default()
                            },
                            ButtonAction::StartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start Game",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

#[derive(Component, Debug)]
struct GameOverSign;

fn despawn_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>,
    game_over_sign_query: Query<Entity, With<GameOverSign>>,
) {
    if let Ok(menu) = menu_query.get_single() {
        commands.entity(menu).despawn_recursive();
    }

    if let Ok(game_over_sign) = game_over_sign_query.get_single() {
        commands.entity(game_over_sign).despawn_recursive();
    }
}

fn handle_menu_buttons(
    interaction_query: Query<(&Interaction, &ButtonAction), With<Button>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::StartGame => next_state.set(GameState::Playing),
            }
        }
    }
}
