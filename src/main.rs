
use std::default;
use bevy::app::AppExit;
use bevy::input::keyboard::{self, Key};
use bevy::ui::debug::print_ui_layout_tree;
use bevy::{asset::transformer, audio::{AddAudioSource, AudioLoader, AudioPlugin}, prelude::*, render::texture, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use rand::random;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NIMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED : f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS :u32= 10 ;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME:f32 = 1.0;
pub const ENEMY_SPAWN_TIME:f32=5.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // .insert_resource(Score{value:0})
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScores>()

        .add_event::<GameOver>()

        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(First, player_movement)
        .add_systems(First, enemy_movement)
        .add_systems(First, confine_player_movement)
        .add_systems(First, confine_enemy_movement)
        .add_systems(First, update_enemy_direction)
        .add_systems(First, update_score)
        .add_systems(First, tick_star_spawn_timer)
        .add_systems(First, tick_enemy_spawn_timer)
        .add_systems(First, spawn_stars_over_time)
        .add_systems(First, spawn_enemies_over_time)
        .add_systems(First, exit_game)
        .add_systems(First, handle_game_over)
        .add_systems(First, update_high_scores)
        .add_systems(First, high_scores_updated)

        .add_systems(First, enemy_hit_player)

        .add_systems(First, player_hit_star)
        .run();
}

#[derive(Component)]
pub struct Enemy{
    // pub speed: f32,
    pub direction: Vec2,
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Star{}

#[derive(Resource)]
pub struct Score{
    pub value: u32,
}
impl Default for Score{
    fn default() -> Self {
        Score{value:0}
    }
}

#[derive(Resource,Debug)]
pub struct HighScores{
    pub scores: Vec<(String ,u32)>,
}
impl Default for HighScores{
    fn default() -> Self {
        Self { scores: Vec::new()}
    }
}
#[derive(Resource)]
pub struct  StarSpawnTimer{
    pub timer:Timer,
}
impl Default for StarSpawnTimer{
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME,TimerMode::Repeating),}
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer{
    pub timer: Timer,
}

impl Default for EnemySpawnTimer{
    fn default() -> EnemySpawnTimer{
        EnemySpawnTimer{
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct GameOver{
    pub score: u32,
}
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width()/2.0, window.height() /2.0, 0.0),
            ..default()
        }
    );
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0, window.height() /2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            }, 
        Player{},
        )
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
     let window =window_query.get_single().unwrap();

    for i in 0..NIMBER_OF_ENEMIES{
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy{
                    // speed: 100.0,
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            
            )
        );
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{},
        ));
    }
}


pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        // Vec3是指三维向量，这里我们创建一个初始值为零的三维向量
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD){
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        // 正则化,防止速度过快
        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform,With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
){
    for (mut transform ,enemy) in enemy_query.iter_mut(){
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // audio: Res<Audio>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut(){
        let mut direction_changed = false;
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed{
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() < 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            
            // 播放音乐
            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::DESPAWN,
            
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut(){
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    // 碰到之后要删掉玩家,所以需要entity
    player_query: Query<(Entity,&Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity,player_transform)) = player_query.get_single(){
        for enemy_transform in enemy_query.iter(){
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Player hit by enemy! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver{score: score.value});
            }
        }
    }
}

// pub fn enemy_hit_player(
//     mut commands: Commands,
//     mut game_over_event_writer: EventWriter<GameOver>,
//     player_query: Query<(Entity, &Transform), With<Player>>,
//     enemy_query: Query<&Transform, With<Enemy>>,
//     asset_server: Res<AssetServer>,
//     score: Res<Score>,
// ) {
//     if let Ok((player_entity, player_transform)) = player_query.get_single() {
//         for enemy_transform in enemy_query.iter() {
//             let distance = player_transform
//                 .translation
//                 .distance(enemy_transform.translation);
//             let player_radius = PLAYER_SIZE / 2.0;
//             let enemy_radius = ENEMY_SIZE / 2.0;
//             if distance < player_radius + enemy_radius {
//                 println!("Enemy hit player! Game Over!");
//                 let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
//                 commands.spawn(AudioBundle {
//                     source: sound_effect,
//                     settings: PlaybackSettings::DESPAWN,
//                 });
//                 commands.entity(player_entity).despawn();
//              // 1111
//                 game_over_event_writer.send(GameOver { score: score.value });
//             }
//         }
//     }
// }

pub fn player_hit_star(
    //碰到之后消灭星星,使用命令
    mut commands: Commands,
    player_query: Query<&Transform,With<Player>>,
    // 碰到之后要删掉星星,需要entity
    mut star_query: Query<(&Transform,Entity), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
){
    if let Ok(player_transform) = player_query.get_single(){
        for (star_trans, star_entity) in star_query.iter(){
            let distance = player_transform
                .translation
                .distance(star_trans.translation);
            let player_radius = PLAYER_SIZE / 2.0 ;
            let star_radius = STAR_SIZE / 2.0 ;

            if distance<player_radius+star_radius{
                println!("Player hit star!");
                score.value+=1;
                commands.entity(star_entity).despawn();
                let sound = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn(AudioBundle{
                    source: sound,
                    settings: PlaybackSettings::default(),
                });
            }
        }
    }
}

pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score :{}",score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
){
    star_spawn_timer.timer.tick(time.delta());
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
){
    enemy_spawn_timer.timer.tick(time.delta());
}
pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
){
    // timer是倒计时,finished用来检测当timer为0时,进入if
    if star_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() *window.width();
        let random_y = random::<f32>() * window.height();
        
        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{},
        ));
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
){
    if enemy_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy{
                direction: Vec2::new(random::<f32>(),random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
){
    if keyboard_input.just_pressed(KeyCode::Escape){
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>){
    for event in game_over_event_reader.read(){
        println!("Your final score is :{}", event.score.to_string());

    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
){
    for event in game_over_event_reader.read(){
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScores>,
){
    if high_scores.is_changed(){
        println!("High Scores: {:?}", high_scores);
    }
}