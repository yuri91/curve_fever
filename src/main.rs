use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod collisions;
mod components;
mod systems;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(systems::NextZIndex(0))
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (
            systems::update_acceleration,
            systems::update_positions,
            systems::update_collisions::<components::Line>,
            systems::update_collisions::<components::Arc>,
            systems::update_paths::<components::Line>,
            systems::update_paths::<components::Arc>,
            systems::update_paths::<components::Head>,
        ).chain())
        .add_systems(PostUpdate, (
            systems::update_translation,
        ))
        .run();
}



