use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

use crate::{
    actions::Actions, asteroids::Asteroid, Collider, GameState, Heading, Hit, Speed, Wrapping,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_ship)
            .add_systems(
                Update,
                (rotate, accelerate, displace, detect_collisions, wrap)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(FixedUpdate, handle_hit.run_if(in_state(GameState::Playing)));
    }
}

const SHIP_COLOR: Color = Color::WHITE;
const ROTATION_SPEED: f32 = 10.;

#[derive(Component)]
pub struct Ship;

#[derive(Bundle)]
pub struct ShipBundle {
    shape: ShapeBundle,
    fill: Fill,
    ship: Ship,
    speed: Speed,
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
            speed: Speed(Vec3::ZERO),
            heading: Heading(Vec3::ZERO),
            wrapping: Wrapping,
            collider: Collider,
        }
    }
}

fn spawn_ship(mut commands: Commands) {
    info!("Spawning ship");

    commands.spawn(ShipBundle::new(30.));
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
        transform
            .rotate_z(actions.player_movement.unwrap().x * ROTATION_SPEED * time.delta_seconds());
    }
}

fn accelerate(
    time: Res<Time>,
    actions: Res<Actions>,
    mut ship_query: Query<(&mut Speed, &mut Heading, &Transform), With<Ship>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut speed, mut heading, transform) in &mut ship_query {
        let direction = actions.player_movement.unwrap().y;
        if direction > 0. {
            let velocity = direction * time.delta_seconds();
            let new_heading = transform.rotation * Vec3::Y;
            let new_speed = speed.0 + new_heading * velocity;

            speed.0 = new_speed;
            heading.0 = new_heading;
        }
    }
}

// Todo: This should probably be extracted as it is the same logic for the asteroids
fn displace(time: Res<Time>, mut ship_query: Query<(&mut Transform, &Speed), With<Ship>>) {
    for (mut transform, speed) in &mut ship_query {
        let translation_delta = 100. * speed.0 * time.delta_seconds();
        transform.translation += translation_delta;
    }
}

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

fn handle_hit(mut commands: Commands, hit_query: Query<(Entity, &Hit), With<Ship>>) {
    for (entity, _) in hit_query.iter() {
        commands.entity(entity).remove::<Hit>();
        info!("Ship destroyed");
    }
}
