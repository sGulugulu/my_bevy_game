pub mod events;
pub mod systems;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use enemy::*;
use player::PlayerPlugin;
use star::*;
use score::*;
use crate::events::*;
use crate::systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // .insert_resource(Score{value:0})
        .add_event::<GameOver>()

        // !!! 子插件可能依赖更高级的时间,因此要在加入事件后加入自己的插件
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(StarPlugin)
        .add_systems(Startup, spawn_camera)

        .add_systems(First, exit_game)
        .add_systems(First, handle_game_over)

        .run();
}


