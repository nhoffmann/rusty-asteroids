use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use player::PlayerPlugin;
use rand::{prelude::thread_rng, Rng};

use actions::ActionsPlugin;
use bullets::BulletsPlugin;
use ship::ShipPlugin;

mod actions;
mod asteroids;
mod bullets;
mod player;
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
            .add_plugins((
                ActionsPlugin,
                ShipPlugin,
                BulletsPlugin,
                AsteroidsPlugin,
                PlayerPlugin,
                ShapePlugin,
            ))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, wrap.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Debug, Clone, Copy, Default)]
struct Heading(Vec3);

#[derive(Component, Debug, Clone, Copy, Default)]
struct Position(Vec2);

#[derive(Component)]
struct Wrapping;

// This vector gives the direction and velocity the entity is travelling in
#[derive(Component, Debug, Clone, Copy)]
struct Velocity(Vec3);

impl Velocity {
    fn random() -> Self {
        Self::random_with_speed(thread_rng().gen_range(0.1..3.))
    }

    fn random_with_speed(speed: f32) -> Self {
        Self(
            Vec3::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                0.,
            ) * speed,
        )
    }
}

#[derive(Component)]
struct Collider;

#[derive(Component, Clone)]
struct Hit;

// Todo: This should be extracted, the asteroids will need the same logic
fn wrap(window: Query<&Window>, mut wrapping_query: Query<&mut Transform, With<Wrapping>>) {
    if let Ok(window) = window.get_single() {
        let (width, height) = (window.width(), window.height());

        for mut transform in &mut wrapping_query {
            let position = transform.translation.truncate();

            if position.x > width / 2. {
                transform.translation = Vec3::new(position.x - width, position.y * -1., 0.)
            }
            if position.x < width / -2. {
                transform.translation = Vec3::new(position.x + width, position.y * -1., 0.)
            }
            if position.y > height / 2. {
                transform.translation = Vec3::new(position.x * -1., position.y - height, 0.);
            }
            if position.y < height / -2. {
                transform.translation = Vec3::new(position.x * -1., position.y + height, 0.);
            }
        }
    }
}
