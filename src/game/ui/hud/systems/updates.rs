use bevy::prelude::*;

use crate::game::score::resources::Score;
use crate::game::ui::hud::components::ScoreText;

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}
