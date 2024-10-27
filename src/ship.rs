use std::time::Duration;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

use crate::{
    actions::Actions, asteroids::Asteroid, Collider, GameState, Heading, Hit, Position, Velocity,
    Wrapping,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<ShipState>()
            .add_systems(OnEnter(ShipState::Flying), spawn_ship)
            .add_systems(
                Update,
                (rotate, accelerate, detect_collisions, gizmo_draw_aiming)
                    .run_if(in_state(ShipState::Flying)),
            )
            .add_systems(
                FixedUpdate,
                (handle_hit, displace).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(ShipState::Destroyed), (despawn_ship, destroy))
            .add_systems(OnEnter(GameState::Menu), despawn_ship)
            .add_systems(Update, respawn_timer.run_if(in_state(ShipState::Destroyed)));
    }
}

const SHIP_COLOR: Color = Color::srgb(0., 1., 0.);
const SHIP_SPEED: f32 = 300.;
const SHIP_RADIUS: f32 = 15.;
const ROTATION_SPEED: f32 = 7.;
const RESPAWN_TIME_IN_SECONDS: u64 = 3;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum ShipState {
    #[default]
    Flying,
    Destroyed,
}

#[derive(Component)]
pub struct Ship;

#[derive(Bundle)]
pub struct ShipBundle {
    shape: ShapeBundle,
    fill: Fill,
    ship: Ship,
    velocity: Velocity,
    heading: Heading,
    wrapping: Wrapping,
    collider: Collider,
}

impl ShipBundle {
    fn new(radius: f32) -> Self {
        let shape = shapes::RegularPolygon {
            sides: 3,
            feature: shapes::RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        };

        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            fill: Fill::color(SHIP_COLOR),
            ship: Ship,
            velocity: Velocity(Vec3::ZERO),
            heading: Heading(Vec3::ZERO),
            wrapping: Wrapping,
            collider: Collider,
        }
    }
}

fn spawn_ship(mut commands: Commands) {
    info!("Spawning ship");

    commands.spawn(ShipBundle::new(SHIP_RADIUS));
}

fn despawn_ship(mut commands: Commands, ship_query: Query<Entity, With<Ship>>) {
    if let Ok(ship) = ship_query.get_single() {
        commands.entity(ship).despawn_recursive();
    }
}

fn rotate(
    time: Res<Time>,
    actions: Res<Actions>,
    mut ship_query: Query<&mut Transform, With<Ship>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for mut transform in &mut ship_query {
        transform.rotate_z(
            (actions.player_movement.unwrap().x * -1.) * ROTATION_SPEED * time.delta_seconds(),
        );
    }
}

fn accelerate(
    time: Res<Time>,
    actions: Res<Actions>,
    mut ship_query: Query<(&mut Velocity, &mut Heading, &Transform), With<Ship>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut velocity, mut heading, transform) in &mut ship_query {
        let direction = actions.player_movement.unwrap().y;
        if direction > 0. {
            let velocity_change = direction * time.delta_seconds();
            let new_heading = transform.rotation * Vec3::Y;
            let new_velocity = velocity.0 + new_heading * velocity_change;

            velocity.0 = new_velocity;
            heading.0 = new_heading;
        }
    }
}

// Todo: This should probably be extracted as it is the same logic for the asteroids
fn displace(time: Res<Time>, mut ship_query: Query<(&mut Transform, &Velocity), With<Ship>>) {
    for (mut transform, velocity) in &mut ship_query {
        let translation_delta = SHIP_SPEED * velocity.0 * time.delta_seconds();
        transform.translation += translation_delta;
    }
}

fn detect_collisions(
    mut commands: Commands,
    ship_query: Query<(Entity, &Transform), With<Ship>>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
) {
    for (ship_entity, ship_transform) in ship_query.iter() {
        let ship_bounding_box =
            Aabb2d::new(ship_transform.translation.truncate(), Vec2::new(15., 15.));

        for (asteroid_entity, asteroid_transform) in asteroid_query.iter() {
            let asteroid_bounding_box = Aabb2d::new(
                asteroid_transform.translation.truncate(),
                Vec2::new(20., 20.),
            );

            if ship_bounding_box.intersects(&asteroid_bounding_box) {
                commands.entity(asteroid_entity).insert(Hit);
                commands.entity(ship_entity).insert(Hit);
            }
        }
    }
}

fn handle_hit(
    mut commands: Commands,
    mut next_state: ResMut<NextState<ShipState>>,
    hit_query: Query<(Entity, &Hit), With<Ship>>,
) {
    for (entity, _) in hit_query.iter() {
        commands.entity(entity).remove::<Hit>();
        next_state.set(ShipState::Destroyed);
    }
}

fn gizmo_draw_aiming(mut gizmos: Gizmos, ship_query: Query<&Transform, With<Ship>>) {
    for &transform in &ship_query {
        let length = 100.;

        gizmos.arrow_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + (transform.rotation * Vec3::Y).truncate() * length,
            SHIP_COLOR,
        );
    }
}

#[derive(Component)]
struct DestroyedShip;

#[derive(Bundle)]
struct DestroyedBundle {
    shape: ShapeBundle,
    fill: Fill,
    destroyed_ship: DestroyedShip,
}

impl DestroyedBundle {
    fn new(position: Position) -> Self {
        let shape = shapes::RegularPolygon {
            sides: 4,
            feature: shapes::RegularPolygonFeature::Radius(SHIP_RADIUS),
            ..shapes::RegularPolygon::default()
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
            fill: Fill::color(SHIP_COLOR),
            destroyed_ship: DestroyedShip,
        }
    }
}

#[derive(Component)]
struct RespawnTime(Timer);

fn destroy(mut commands: Commands, ship_query: Query<&Transform, With<Ship>>) {
    if let Ok(transform) = ship_query.get_single() {
        commands.spawn(DestroyedBundle::new(Position(
            transform.translation.truncate(),
        )));
        commands.spawn(RespawnTime(Timer::new(
            Duration::from_secs(RESPAWN_TIME_IN_SECONDS),
            TimerMode::Once,
        )));
        info!("Ship destroyed")
    }
}

fn respawn_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut RespawnTime)>,
    ship_query: Query<Entity, With<DestroyedShip>>,
    mut next_state: ResMut<NextState<ShipState>>,
) {
    for (entity, mut respawn_timer) in &mut timer_query {
        respawn_timer.0.tick(time.delta());

        if respawn_timer.0.finished() {
            commands.entity(entity).despawn();
            let ship = ship_query.single();
            commands.entity(ship).despawn_recursive();

            next_state.set(ShipState::Flying);
        }
    }
}
