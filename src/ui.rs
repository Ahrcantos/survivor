use bevy::{
    prelude::{
        App, Commands, Component, Plugin, Query, Res, Resource, Startup, TextBundle, Update, With,
    },
    text::{Text, TextStyle},
};

#[derive(Resource, Default)]
pub struct Score(u32);

impl Score {
    pub fn inc(&mut self) {
        self.0 += 1;
    }

    pub fn count(&self) -> u32 {
        self.0
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup_score)
            .add_systems(Update, update_score_text);
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_score(mut commands: Commands, score: Res<Score>) {
    let text = format!("Score: {}", score.count());
    commands.spawn((
        TextBundle::from_section(
            &text,
            TextStyle {
                font_size: 20.0,
                ..Default::default()
            },
        ),
        ScoreText,
    ));
}

fn update_score_text(mut text_component: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    let text = format!("Score: {}", score.count());
    let mut text_component = text_component.single_mut();

    text_component.sections[0].value = text;
}
