use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use actions::ActionsPlugin;
use bullets::BulletsPlugin;
use ship::ShipPlugin;

mod actions;
mod bullets;
mod ship;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Playing,
}

pub struct Asteroids;

impl Plugin for Asteroids {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((ActionsPlugin, ShipPlugin, BulletsPlugin, ShapePlugin))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Debug, Clone, Copy, Default)]
struct Heading(Vec3);

#[derive(Component, Debug, Clone, Copy, Default)]
struct Position(Vec2);
