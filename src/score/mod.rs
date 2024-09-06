use bevy::prelude::*;

pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .add_systems(First, update_score)
        .add_systems(First, update_high_scores)
        .add_systems(First, high_scores_updated);
    }
}