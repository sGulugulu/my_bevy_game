use bevy::prelude::*;
use systems::*;

pub mod components;
pub mod systems;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(First, player_movement)
        .add_systems(First, confine_player_movement)
        .add_systems(First, enemy_hit_player)
        .add_systems(First, player_hit_star);

    }
}