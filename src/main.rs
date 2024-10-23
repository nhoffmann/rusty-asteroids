use bevy::prelude::*;

use asteroids::Asteroids;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Asteroids)
        .run();
}
