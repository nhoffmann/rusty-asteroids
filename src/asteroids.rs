use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};
use rand::{prelude::thread_rng, Rng};

use crate::{Collider, GameState, Hit, Position, Velocity, Wrapping};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_asteroids)
            .add_systems(
                Update,
                (displace, gizmo_draw_travelling_directions).run_if(in_state(GameState::Playing)),
            )
            .add_systems(FixedUpdate, handle_hit.run_if(in_state(GameState::Playing)));
    }
}

const ASTEROID_RADIUS_LARGE: f32 = 40.;
const ASTEROID_RADIUS_MEDIUM: f32 = 20.;
const ASTEROID_RADIUS_SMALL: f32 = 10.;
const ASTEROID_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    fn radius(&self) -> f32 {
        match self {
            Self::Large => ASTEROID_RADIUS_LARGE,
            Self::Medium => ASTEROID_RADIUS_MEDIUM,
            Self::Small => ASTEROID_RADIUS_SMALL,
        }
    }
}

#[derive(Bundle)]
struct AsteroidBundle {
    shape: ShapeBundle,
    fill: Fill,
    asteroid: Asteroid,
    velocity: Velocity,
    wrapping: Wrapping,
    collider: Collider,
    size: AsteroidSize,
}

impl AsteroidBundle {
    fn new(position: Position, velocity: Velocity, size: AsteroidSize) -> Self {
        let shape = shapes::Circle {
            radius: size.radius(),
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
            velocity,
            wrapping: Wrapping,
            collider: Collider,
            size,
        }
    }

    fn random_velocity(position: Position, size: AsteroidSize) -> Self {
        let random_velocity = Velocity(
            Vec3::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                0.,
            ) * thread_rng().gen_range(0.1..3.),
        );
        Self::new(position, random_velocity, size)
    }
}

fn spawn_asteroids(mut commands: Commands, window: Query<&Window>) {
    info!("Spawning asteroids");
    let window = window.single();

    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    let rand_num_asteroids = thread_rng().gen_range(7..13);

    for _ in 0..rand_num_asteroids {
        let random_x: f32 = thread_rng().gen_range(half_width * -1.0..half_width);
        let random_y: f32 = thread_rng().gen_range(half_height * -1.0..half_height);
        let random_position = Position(Vec2::new(random_x, random_y));

        commands.spawn(AsteroidBundle::random_velocity(
            random_position,
            AsteroidSize::Large,
        ));
    }
}

fn displace(mut asteroid_query: Query<(&mut Transform, &Velocity), With<Asteroid>>) {
    for (mut transform, velocity) in &mut asteroid_query {
        let translation_delta = velocity.0;
        transform.translation += translation_delta;
    }
}

fn handle_hit(
    mut commands: Commands,
    hit_query: Query<(Entity, &AsteroidSize, &Transform, &Hit), With<Asteroid>>,
) {
    for (entity, size, transform, _) in hit_query.iter() {
        match size {
            AsteroidSize::Large => {
                commands.spawn(AsteroidBundle::random_velocity(
                    Position(transform.translation.truncate()),
                    AsteroidSize::Medium,
                ));
                commands.spawn(AsteroidBundle::random_velocity(
                    Position(transform.translation.truncate()),
                    AsteroidSize::Medium,
                ));
            }
            AsteroidSize::Medium => {
                commands.spawn(AsteroidBundle::random_velocity(
                    Position(transform.translation.truncate()),
                    AsteroidSize::Small,
                ));
                commands.spawn(AsteroidBundle::random_velocity(
                    Position(transform.translation.truncate()),
                    AsteroidSize::Small,
                ));
            }
            _ => (),
        }

        commands.entity(entity).despawn_recursive();
    }
}

fn gizmo_draw_travelling_directions(
    mut gizmos: Gizmos,
    ship_query: Query<(&Transform, &Velocity), With<Asteroid>>,
) {
    for (&transform, velocity) in &ship_query {
        let length = 100.;

        gizmos.arrow_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + velocity.0.truncate() * length,
            Color::WHITE,
        );
    }
}
