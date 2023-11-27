mod ability;
mod enemy;
mod mouse_position;
mod player;
mod ui;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::ability::AbilityPlugin;
use self::enemy::EnemyPlugin;
use self::mouse_position::MousePositionPlugin;
use self::player::PlayerPlugin;
use self::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PlayerPlugin,
            EnemyPlugin,
            MousePositionPlugin,
            UiPlugin,
            AbilityPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            #[cfg(debug_assertions)]
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}
