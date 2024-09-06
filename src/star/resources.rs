use bevy::{prelude::Resource, time::{Timer, TimerMode}};

use super::STAR_SPAWN_TIME;


#[derive(Resource)]
pub struct  StarSpawnTimer{
    pub timer:Timer,
}
impl Default for StarSpawnTimer{
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME,TimerMode::Repeating),}
    }
}
