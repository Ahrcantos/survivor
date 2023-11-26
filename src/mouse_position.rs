use bevy::{
    prelude::{
        App, Camera, Commands, GlobalTransform, Plugin, Query, ResMut, Resource, Startup, Update,
        Vec2, With,
    },
    window::{PrimaryWindow, Window},
};

use crate::MainCamera;

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_mouse_world_position);
    }
}

#[derive(Resource, Default)]
pub struct MouseWorldPosition(pub Vec2);

fn setup(mut commands: Commands) {
    commands.init_resource::<MouseWorldPosition>();
}

fn update_mouse_world_position(
    mut mouse_position: ResMut<MouseWorldPosition>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.0 = world_position;
    }
}
