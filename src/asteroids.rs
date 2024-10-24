use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};
use rand::{prelude::thread_rng, Rng};

use crate::{GameState, Position, Speed, Wrapping};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_asteroids)
            .add_systems(Update, (displace).run_if(in_state(GameState::Playing)));
    }
}

const ASTEROID_RADIUS: f32 = 40.;
const ASTEROID_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct Asteroid;

#[derive(Bundle)]
struct AsteroidBundle {
    shape: ShapeBundle,
    fill: Fill,
    asteroid: Asteroid,
    speed: Speed,
    wrapping: Wrapping,
}

impl AsteroidBundle {
    fn new(position: Position, speed: Speed) -> Self {
        let shape = shapes::Circle {
            radius: ASTEROID_RADIUS,
            center: Vec2::ZERO,
        };

        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: Vec3::from((position.0, 0.)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            fill: Fill::color(ASTEROID_COLOR),
            asteroid: Asteroid,
            speed,
            wrapping: Wrapping,
        }
    }
}

fn spawn_asteroids(mut commands: Commands, window: Query<&Window>) {
    info!("Spawn asteroids");
    let window = window.single();

    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    let rand_num_asteroids = thread_rng().gen_range(7..13);
    for _ in 0..rand_num_asteroids {
        let random_x: f32 = thread_rng().gen_range(half_width * -1.0..half_width);
        let random_y: f32 = thread_rng().gen_range(half_height * -1.0..half_height);
        let random_position = Position(Vec2::new(random_x, random_y));

        let random_speed = Speed(Vec3::new(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..1.),
            0.,
        ));
        info!(
            "Spawning asteroid: {:?}, {:?}",
            random_position, random_speed
        );
        commands.spawn(AsteroidBundle::new(random_position, random_speed));
    }
}

fn displace(mut asteroid_query: Query<(&mut Transform, &Speed), With<Asteroid>>) {
    for (mut transform, speed) in &mut asteroid_query {
        let translation_delta = speed.0;
        transform.translation += translation_delta;
        // info!("{:?}", transform.translation);
    }
}
