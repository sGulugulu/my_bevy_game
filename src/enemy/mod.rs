use bevy::prelude::*;
use resources::EnemySpawnTimer;

pub mod components;
pub mod resources;
pub mod systems;

use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED : f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME:f32=5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
        .add_systems(Startup, spawn_enemies)
        .add_systems(First, enemy_movement)
        .add_systems(First, confine_enemy_movement)
        .add_systems(First, update_enemy_direction)
        .add_systems(First, tick_enemy_spawn_timer)
        .add_systems(First, spawn_enemies_over_time);
    }
}