use std::{ops::Range, time::Duration};

use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};
use rand::{prelude::thread_rng, Rng};

use crate::{Collider, GameState, Hit, Position, Velocity, Wrapping};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<AsteroidsState>()
            .add_systems(OnEnter(AsteroidsState::Flying), spawn_asteroids)
            .add_systems(
                Update,
                (gizmo_draw_travelling_directions, check_level_complete)
                    .run_if(in_state(AsteroidsState::Flying)),
            )
            .add_systems(
                FixedUpdate,
                (displace, handle_hit).run_if(in_state(AsteroidsState::Flying)),
            )
            .add_systems(
                Update,
                respawn_timer.run_if(in_state(AsteroidsState::Destroyed)),
            )
            .add_systems(OnEnter(AsteroidsState::Destroyed), start_respawn_timer);
    }
}

const ASTEROID_RADIUS_LARGE: f32 = 40.;
const ASTEROID_RADIUS_MEDIUM: f32 = 20.;
const ASTEROID_RADIUS_SMALL: f32 = 10.;
const ASTEROID_COLOR: Color = Color::WHITE;
const ASTEROID_RESPAWN_TIME_IN_SECONDS: u64 = 4;
const ASTEROID_SPAWN_RANGE: Range<i32> = 5..10;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum AsteroidsState {
    #[default]
    Flying,
    Destroyed,
}

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub enum AsteroidSize {
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
        Self::new(position, Velocity::random(), size)
    }
}

fn spawn_asteroids(mut commands: Commands, window: Query<&Window>) {
    info!("Spawning asteroids");
    let window = window.single();

    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    let rand_num_asteroids = thread_rng().gen_range(ASTEROID_SPAWN_RANGE);

    for _ in 0..rand_num_asteroids {
        let random_x: f32 = thread_rng().gen_range(half_width * -1.0..half_width);
        let random_y: f32 = thread_rng().gen_range(half_height * -1.0..half_height);
        let random_position = Position(Vec2::new(random_x, random_y));

        commands.spawn(AsteroidBundle::new(
            random_position,
            // initial asteroids shouldn't be too fast
            Velocity::random_with_speed(1.),
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

fn check_level_complete(
    mut next_state: ResMut<NextState<AsteroidsState>>,
    asteroids_query: Query<&Asteroid>,
) {
    if asteroids_query.is_empty() {
        next_state.set(AsteroidsState::Destroyed)
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

#[derive(Component)]
struct RespawnTime(Timer);

fn start_respawn_timer(mut commands: Commands) {
    commands.spawn(RespawnTime(Timer::new(
        Duration::from_secs(ASTEROID_RESPAWN_TIME_IN_SECONDS),
        TimerMode::Once,
    )));
    info!("Level complete")
}

fn respawn_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut RespawnTime)>,
    mut next_state: ResMut<NextState<AsteroidsState>>,
) {
    for (entity, mut respawn_timer) in &mut timer_query {
        respawn_timer.0.tick(time.delta());

        if respawn_timer.0.finished() {
            info!("Asteroid respawn timer finished");
            commands.entity(entity).despawn();

            next_state.set(AsteroidsState::Flying);
        }
    }
}
