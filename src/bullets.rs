use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};

use crate::{actions::FiredAction, GameState, Heading, Position};

const BULLET_RADIUS: f32 = 2.;
const BULLET_SPEED: f32 = 10.;
const BULLET_COLOR: Color = Color::WHITE;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bullet, displace, despawn_bullet).run_if(in_state(GameState::Playing)),
        );
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
}

impl BulletBundle {
    fn new(heading: Heading, position: Position) -> Self {
        let shape = shapes::Circle {
            radius: BULLET_RADIUS,
            center: position.0,
        };

        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            fill: Fill::color(BULLET_COLOR),
            bullet: Bullet,
            heading,
            origin: position,
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

        let distance = translation.distance(Vec3::from((origin.0, 0.)));
        if distance > 1000. {
            commands.entity(entity).despawn_recursive();
        }
    }
}
