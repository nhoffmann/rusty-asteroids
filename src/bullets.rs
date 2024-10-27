use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};

use crate::{
    actions::FiredAction, asteroids::Asteroid, Collider, GameState, Heading, Hit, Position,
};

const BULLET_RADIUS: f32 = 2.;
const BULLET_SPEED: f32 = 10.;
const BULLET_COLOR: Color = Color::WHITE;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bullet, detect_collisions, despawn_bullet).run_if(in_state(GameState::Playing)),
        )
        .add_systems(FixedUpdate, displace.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
struct BulletBundle {
    shape: ShapeBundle,
    fill: Fill,
    bullet: Bullet,
    heading: Heading,
    origin: Position,
    collider: Collider,
}

impl BulletBundle {
    fn new(heading: Heading, position: Position) -> Self {
        let shape = shapes::Circle {
            radius: BULLET_RADIUS,
            center: Vec2::ZERO,
        };

        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: position.0.extend(0.),
                        ..default()
                    },
                    ..default()
                },

                ..default()
            },
            fill: Fill::color(BULLET_COLOR),
            bullet: Bullet,
            heading,
            origin: position,
            collider: Collider,
        }
    }
}

fn spawn_bullet(mut commands: Commands, actions: Res<FiredAction>) {
    if actions.heading.is_none() && actions.position.is_none() {
        return;
    }

    commands.spawn(BulletBundle::new(
        actions.heading.unwrap(),
        actions.position.unwrap(),
    ));
}

fn displace(mut bullet_query: Query<(&mut Transform, &Heading), With<Bullet>>) {
    for (mut transform, heading) in &mut bullet_query {
        let translation_delta = heading.0 * BULLET_SPEED;
        transform.translation += translation_delta;
    }
}

fn despawn_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Position), With<Bullet>>,
) {
    for (entity, transform, origin) in bullet_query.iter() {
        let translation = transform.translation;

        let distance = translation.distance(origin.0.extend(0.));
        if distance > 1000. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn detect_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_bounding_box =
            Aabb2d::new(bullet_transform.translation.truncate(), Vec2::new(1., 1.));

        for (entity, asteroid_transform) in asteroid_query.iter() {
            let asteroid_bounding_box = Aabb2d::new(
                asteroid_transform.translation.truncate(),
                Vec2::new(20., 20.),
            );

            if bullet_bounding_box.intersects(&asteroid_bounding_box) {
                commands.entity(entity).insert(Hit);
                commands.entity(bullet_entity).despawn_recursive();
            }
        }
    }
}
