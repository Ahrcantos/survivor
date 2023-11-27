use std::time::Duration;

use bevy::{
    prelude::{
        App, AssetServer, Assets, AtlasImageBundle, BuildChildren, Commands, Component, NodeBundle,
        Plugin, Query, Res, ResMut, Resource, Startup, Timer, Update, Vec2, With,
    },
    sprite::TextureAtlas,
    time::{Time, TimerMode},
    ui::{JustifyContent, PositionType, Style, UiTextureAtlasImage, Val},
};

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DashAbilityCooldown>()
            .add_systems(Startup, setup_ability_ui)
            .add_systems(Update, (update_ability_timers, update_ability_ui));
    }
}

#[derive(Resource)]
pub struct DashAbilityCooldown {
    timer: Option<Timer>,
}

impl DashAbilityCooldown {
    const COOLDOWN_TIME: f32 = 1.0;

    pub fn available(&self) -> bool {
        if let Some(timer) = &self.timer {
            timer.finished()
        } else {
            true
        }
    }

    pub fn consume(&mut self) {
        if let Some(timer) = &mut self.timer {
            if timer.finished() {
                timer.reset();
            }
        } else {
            self.timer = Some(Timer::from_seconds(Self::COOLDOWN_TIME, TimerMode::Once))
        }
    }

    pub fn percentage(&self) -> f32 {
        if let Some(timer) = &self.timer {
            timer.percent()
        } else {
            1.0
        }
    }

    pub fn tick(&mut self, delta: f32) {
        if let Some(timer) = &mut self.timer {
            timer.tick(Duration::from_secs_f32(delta));
        }
    }
}

impl Default for DashAbilityCooldown {
    fn default() -> Self {
        Self { timer: None }
    }
}

fn update_ability_timers(time: Res<Time>, mut dash: ResMut<DashAbilityCooldown>) {
    dash.tick(time.delta_seconds());
}

#[derive(Component)]
struct DashCooldownIcon;

fn setup_ability_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/ui/abilities.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(AtlasImageBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(25.0),
                    bottom: Val::Px(25.0),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone_weak(),
                texture_atlas_image: UiTextureAtlasImage {
                    index: 0,
                    flip_x: false,
                    flip_y: false,
                },
                ..Default::default()
            });

            parent.spawn((
                AtlasImageBundle {
                    style: Style {
                        width: Val::Px(80.0),
                        height: Val::Px(80.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(25.0),
                        bottom: Val::Px(25.0),
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle,
                    texture_atlas_image: UiTextureAtlasImage {
                        index: 56,
                        flip_x: false,
                        flip_y: false,
                    },
                    ..Default::default()
                },
                DashCooldownIcon,
            ));
        });
}

fn update_ability_ui(
    mut dash_icon: Query<&mut UiTextureAtlasImage, With<DashCooldownIcon>>,
    dash_cooldown: Res<DashAbilityCooldown>,
) {
    let mut dash_icon = dash_icon.single_mut();

    if dash_cooldown.available() {
        dash_icon.index = 55;
    } else {
        let percentage = dash_cooldown.percentage();
        let index = if percentage < 0.125 {
            56
        } else if percentage < 0.25 {
            57
        } else if percentage < 0.375 {
            58
        } else if percentage < 0.5 {
            59
        } else if percentage < 0.6125 {
            60
        } else if percentage < 0.75 {
            61
        } else if percentage < 0.875 {
            62
        } else if percentage < 1.0 {
            63
        } else {
            55
        };

        dash_icon.index = index;
    }
}
