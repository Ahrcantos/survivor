use std::time::Duration;

use bevy::{
    prelude::{
        App, AssetServer, Commands, Component, IntoSystemConfigs, Plugin, Query, Res, Transform,
        Update, Vec3, With, Without,
    },
    sprite::SpriteBundle,
    time::{common_conditions::on_timer, Time},
};
use bevy_rapier2d::prelude::{Collider, Sensor};

use rand::{thread_rng, Rng};

use crate::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_move,
                enemy_spawn.run_if(on_timer(Duration::from_millis(600))),
            ),
        );
    }
}

#[derive(Component)]
pub struct Enemy;

fn enemy_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x = thread_rng().gen_range(-1000.0..1000.0);
    let y = thread_rng().gen_range(-1000.0..1000.0);

    commands.spawn((
        Enemy,
        SpriteBundle {
            texture: asset_server.load("sprites/enemies/basic.png"),
            transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(2.0, 2.0, 2.0)),
            ..Default::default()
        },
        Collider::cuboid(6.0, 8.0),
        Sensor,
    ));
}

const ENEMY_SPEED: f32 = 100.0;

fn enemy_move(
    player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_transforms: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_transform.single();
    let enemy_transforms = enemy_transforms.iter_mut();

    let player_position = player_transform.translation;

    for mut enemy_transform in enemy_transforms {
        let enemy_position = enemy_transform.translation;
        let direction = player_position - enemy_position;
        let direction = direction.normalize();

        enemy_transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
