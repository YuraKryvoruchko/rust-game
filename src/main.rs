use bevy::prelude::*;

const PLAYER_SPAWN_HEIGHT: f32 = -400.0;
const PLAYER_MOVE_SPEED: f32 = 250.0;

const LAZER_SPEED: f32 = 400.0;
const LAZER_Y_OFFSET: f32 = 40.0;
const LAZER_LAYER: f32 = -1.0;

const ASTEROID_MOVE_SPEED: f32 = 250.0;
const ASTEROID_SPAWN_HEIGHT: f32 = 500.0;
const ASTEROID_SPAWN_DIAPASON: Vec2 = Vec2::new(-200.0, 200.0);

#[derive(Component)]
struct Direction {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Lazer;

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct AsteroidSpawTimer(Timer);
#[derive(Resource)]
struct LazerShootingTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AsteroidSpawTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(LazerShootingTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .add_systems(Startup, startup)
        .add_systems(Update, (handle_input, lazer_shooting, spawn_asteroid, move_objects).chain())
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("playerShip1_blue.png")),
        Transform::from_xyz(0.0, PLAYER_SPAWN_HEIGHT, 0.0),
        Speed(PLAYER_MOVE_SPEED),
        Direction {x: 0.0, y: 0.0},
        Player
    ));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut directions: Query<(&mut Direction, &Player)>) {
    for (mut dir, _p) in &mut directions {
        if input.pressed(KeyCode::KeyA) { 
            dir.x = -1.0;
        }
        else if input.pressed(KeyCode::KeyD) {
            dir.x = 1.0; 
        }
        else {
            dir.x = 0.0;
        }
    }
}

fn move_objects(time: Res<Time>, mut transforms: Query<(&mut Transform, &Direction, &Speed)>) {
    for (mut transform, dir, speed) in &mut transforms {
        transform.translation.x += dir.x * speed.0 * time.delta_secs();
        transform.translation.y += dir.y * speed.0 * time.delta_secs();
    }
}

fn lazer_shooting(time: Res<Time>, mut timer: ResMut<LazerShootingTimer>, input: Res<ButtonInput<KeyCode>>, player: Query<&Transform, With<Player>>, 
    asset_server: Res<AssetServer>, mut commands: Commands) {

    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if input.just_pressed(KeyCode::Space) {
        for player_transform in &player {
            commands.spawn((
                Sprite::from_image(asset_server.load("laserBlue03.png")),
                Transform::from_xyz(player_transform.translation.x, player_transform.translation.y + LAZER_Y_OFFSET, LAZER_LAYER),
                Speed(LAZER_SPEED),
                Direction {x: 0.0, y: 1.0}
            ));
        }
        timer.0.reset();
    }
}

fn spawn_asteroid(time: Res<Time>, mut timer: ResMut<AsteroidSpawTimer>, mut commands: Commands, asset_server: Res<AssetServer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load("meteorGrey_big3.png")),
            Transform::from_xyz(rand::random_range(ASTEROID_SPAWN_DIAPASON.x..=ASTEROID_SPAWN_DIAPASON.y), ASTEROID_SPAWN_HEIGHT, 0.0),
            Speed(ASTEROID_MOVE_SPEED),
            Direction {x: 0.0, y: -1.0}
        ));
    }
}
