use bevy::{
    prelude::{
        App, AssetServer, Commands, Component, Entity, Input, KeyCode, MouseButton, Plugin, Query,
        Res, ResMut, Startup, Transform, Update, Vec2, Vec3, With, Without,
    },
    sprite::SpriteBundle,
    time::Time,
};
use bevy_rapier2d::prelude::{Collider, RapierContext, Sensor};

use crate::{enemy::Enemy, mouse_position::MouseWorldPosition, ui::Score};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_spawn).add_systems(
            Update,
            (
                player_move,
                player_dash,
                player_attack,
                projectile_move,
                despawn_out_of_range_projectiles,
                check_projectile_enemy_collision,
                check_lose_condition,
            ),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

fn player_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("sprites/player/player_static.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(2.0, 2.0, 2.0)),
            ..Default::default()
        },
        Collider::cuboid(5.0, 7.0),
        Sensor,
        Direction::East,
    ));
}

const PLAYER_SPEED: f32 = 100.0;

fn player_move(
    mut player_query: Query<(&mut Transform, &mut Direction), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_transform, mut player_direction) = player_query.single_mut();

    // Directions
    if keyboard_input.pressed(KeyCode::W) && keyboard_input.pressed(KeyCode::A) {
        *player_direction = Direction::NorthEast;
    } else if keyboard_input.pressed(KeyCode::W) && keyboard_input.pressed(KeyCode::D) {
        *player_direction = Direction::NorthWest;
    } else if keyboard_input.pressed(KeyCode::W) {
        *player_direction = Direction::North;
    } else if keyboard_input.pressed(KeyCode::S) && keyboard_input.pressed(KeyCode::A) {
        *player_direction = Direction::SouthEast;
    } else if keyboard_input.pressed(KeyCode::S) && keyboard_input.pressed(KeyCode::D) {
        *player_direction = Direction::SouthWest;
    } else if keyboard_input.pressed(KeyCode::S) {
        *player_direction = Direction::South;
    } else if keyboard_input.pressed(KeyCode::A) {
        *player_direction = Direction::East;
    } else if keyboard_input.pressed(KeyCode::D) {
        *player_direction = Direction::West;
    }

    // Movement
    if keyboard_input.pressed(KeyCode::W) {
        player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::A) {
        player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) {
        player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
}

const DASH_DISTANCE: f32 = 60.0;

fn player_dash(
    mut player_query: Query<(&mut Transform, &Direction), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut player_transform, player_direction) = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        let offset = match player_direction {
            Direction::North => Vec3::new(0.0, DASH_DISTANCE, 0.0),
            Direction::South => Vec3::new(0.0, -DASH_DISTANCE, 0.0),
            Direction::East => Vec3::new(-DASH_DISTANCE, 0.0, 0.0),
            Direction::West => Vec3::new(DASH_DISTANCE, 0.0, 0.0),
            Direction::NorthEast => Vec3::new(-DASH_DISTANCE, DASH_DISTANCE, 0.0),
            Direction::NorthWest => Vec3::new(DASH_DISTANCE, DASH_DISTANCE, 0.0),
            Direction::SouthEast => Vec3::new(-DASH_DISTANCE, -DASH_DISTANCE, 0.0),
            Direction::SouthWest => Vec3::new(DASH_DISTANCE, -DASH_DISTANCE, 0.0),
        };

        player_transform.translation += offset;
    }
}

fn player_attack(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mouse_world_position: Res<MouseWorldPosition>,
    player_position: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let player_position = player_position.single().translation.truncate();
    let mouse_world_position = mouse_world_position.into_inner().0;

    let angle = {
        let p_to_c = player_position - mouse_world_position;
        Vec2::Y.angle_between(p_to_c)
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        let mut transform = Transform::from_xyz(player_position.x, player_position.y, 0.0);
        transform.rotate_z(angle);
        commands.spawn((
            Projectile,
            SpriteBundle {
                transform,
                texture: asset_server.load("sprites/projectiles/fire_blast.png"),
                ..Default::default()
            },
            Collider::ball(8.0),
            Sensor,
        ));
    }
}

const PROJECTILE_SPEED: f32 = 1000.0;

fn projectile_move(
    mut projectile_transforms: Query<&mut Transform, With<Projectile>>,
    time: Res<Time>,
) {
    for mut projectile_transform in projectile_transforms.iter_mut() {
        let direction = -projectile_transform.local_y();

        projectile_transform.translation += direction * PROJECTILE_SPEED * time.delta_seconds();
    }
}

fn check_projectile_enemy_collision(
    mut commands: Commands,
    projectiles: Query<Entity, (With<Projectile>, Without<Enemy>)>,
    enemies: Query<Entity, (With<Enemy>, Without<Projectile>)>,
    rapier_context: Res<RapierContext>,
    mut score: ResMut<Score>,
) {
    for enemy in enemies.iter() {
        for projectile in projectiles.iter() {
            if let Some(_) = rapier_context.intersection_pair(enemy, projectile) {
                commands.entity(enemy).despawn();
                commands.entity(projectile).despawn();
                score.inc();
            }
        }
    }
}

fn despawn_out_of_range_projectiles(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), (With<Projectile>, Without<Player>)>,
    player_transform: Query<&Transform, (With<Player>, Without<Projectile>)>,
) {
    let player_position = player_transform.single().translation;

    for (projectile, projectile_transform) in projectiles.iter() {
        let projectile_position = projectile_transform.translation;
        let distance = (player_position - projectile_position).length();

        if distance > 2500.0 {
            commands.entity(projectile).despawn();
        }
    }
}

fn check_lose_condition(
    player: Query<Entity, (With<Player>, Without<Enemy>)>,
    enemies: Query<Entity, (With<Enemy>, Without<Player>)>,
    rapier_context: Res<RapierContext>,
) {
    let player = player.single();

    for enemy in enemies.iter() {
        if let Some(_) = rapier_context.intersection_pair(enemy, player) {
            std::process::exit(0);
        }
    }
}
