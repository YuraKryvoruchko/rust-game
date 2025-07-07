use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    Left,
    Right,
    None
}

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct AsteroidSpawTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AsteroidSpawTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, startup)
        .add_systems(Update, (handle_input, move_player, spawn_asteroid).chain())
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("playerShip1_blue.png")),
        Transform::from_xyz(0.0, -400.0, 0.0),
        Speed(150.0),
        Direction::Left
    ));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut directions: Query<&mut Direction>) {
    for mut direction in &mut directions {
        if input.pressed(KeyCode::KeyA) { 
            *direction = Direction::Left; 
        }
        else if input.pressed(KeyCode::KeyD) {
            *direction = Direction::Right;
        }
        else {
            *direction = Direction::None;
        }
    }
}

fn move_player(time: Res<Time>, mut player_position: Query<(&mut Direction, &mut Transform, &Speed)>) {
    for (dir, mut transform, speed) in &mut player_position {
        match *dir {
            Direction::Left => transform.translation.x -= speed.0 * time.delta_secs(),
            Direction::Right => transform.translation.x += speed.0 * time.delta_secs(),
            _ => (),
        }
    }
}

fn spawn_asteroid(time: Res<Time>, mut timer: ResMut<AsteroidSpawTimer>, mut commands: Commands, asset_server: Res<AssetServer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load("meteorGrey_big3.png")),
            Transform::from_xyz(rand::random_range(-200.0..=200.0), 0.0, 0.0)
        ));
    }
}