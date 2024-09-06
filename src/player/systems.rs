use bevy::{prelude::*, window::PrimaryWindow};

use crate::{enemy::components::*, score::resources::Score, star::components::Star, GameOver, ENEMY_SIZE, STAR_SIZE};

use super::{components::Player, PLAYER_SIZE, PLAYER_SPEED};



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


pub fn player_hit_star(
    //碰到之后消灭星星,使用命令
    mut commands: Commands,
    player_query: Query<&Transform,With<Player>>,
    // 碰到之后要删掉星星,需要entity
    star_query: Query<(&Transform,Entity), With<Star>>,
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

