use bevy::prelude::*;

use crate::{
    asteroids::{BangLargeEvent, BangMediumEvent, BangSmallEvent},
    bullets::BulletFiredEvent,
    ship::ThrustEvent,
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sounds)
            .add_systems(
                Update,
                (fire_laser_sound).run_if(on_event::<BulletFiredEvent>()),
            )
            .add_systems(Update, (thrust_sound).run_if(on_event::<ThrustEvent>()))
            .add_systems(
                Update,
                (bang_large_sound).run_if(on_event::<BangLargeEvent>()),
            )
            .add_systems(
                Update,
                (bang_medium_sound).run_if(on_event::<BangMediumEvent>()),
            )
            .add_systems(
                Update,
                (bang_small_sound).run_if(on_event::<BangSmallEvent>()),
            );
    }
}

#[derive(Resource, Deref)]
struct FireLaserSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
struct ThrustSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
struct BangLargeSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
struct BangMediumSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
struct BangSmallSound(pub Handle<AudioSource>);

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fire_laser_sound_handle = asset_server.load("sounds/fire.ogg");
    commands.insert_resource(FireLaserSound(fire_laser_sound_handle));

    let thrust_sound_handle = asset_server.load("sounds/thrust.ogg");
    commands.insert_resource(ThrustSound(thrust_sound_handle));

    let bang_large_sound_handle = asset_server.load("sounds/bangLarge.ogg");
    commands.insert_resource(BangLargeSound(bang_large_sound_handle));
    let bang_medium_sound_handle = asset_server.load("sounds/bangMedium.ogg");
    commands.insert_resource(BangMediumSound(bang_medium_sound_handle));
    let bang_small_sound_handle = asset_server.load("sounds/bangSmall.ogg");
    commands.insert_resource(BangSmallSound(bang_small_sound_handle));
}

fn fire_laser_sound(
    mut commands: Commands,
    mut event_reader: EventReader<BulletFiredEvent>,
    sound: Res<FireLaserSound>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn thrust_sound(
    mut commands: Commands,
    mut event_reader: EventReader<ThrustEvent>,
    sound: Res<ThrustSound>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn bang_large_sound(
    mut commands: Commands,
    mut event_reader: EventReader<BangLargeEvent>,
    sound: Res<BangLargeSound>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
fn bang_medium_sound(
    mut commands: Commands,
    mut event_reader: EventReader<BangMediumEvent>,
    sound: Res<BangMediumSound>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
fn bang_small_sound(
    mut commands: Commands,
    mut event_reader: EventReader<BangSmallEvent>,
    sound: Res<BangSmallSound>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

// fn play_sound_on_event<E: Event, S: Resource>(
//     mut commands: Commands,
//     mut event_reader: EventReader<E>,
//     sound: Res<S>,
// ) {
//     if !event_reader.is_empty() {
//         event_reader.clear();

//         let sound: Handle<AudioSource> = sound;

//         commands.spawn(AudioBundle {
//             source: sound.clone(),
//             settings: PlaybackSettings::DESPAWN,
//         });
//     }
// }
