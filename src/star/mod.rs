use bevy::prelude::*;
use resources::*;
use systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub const NUMBER_OF_STARS :u32= 10 ;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME:f32 = 1.0;

pub struct StarPlugin ;

impl Plugin for StarPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, spawn_stars)
        
        .add_systems(First, tick_star_spawn_timer)
        .add_systems(First, spawn_stars_over_time);



    }
}