use bevy::prelude::*;

#[derive(Component)]
struct Direction {
    x: f32,
    y: f32
}

#[derive(Component)]
struct PlayerComponent;

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct AsteroidSpawTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AsteroidSpawTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, startup)
        .add_systems(Update, (handle_input, spawn_asteroid, move_objects).chain())
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("playerShip1_blue.png")),
        Transform::from_xyz(0.0, -400.0, 0.0),
        Speed(150.0),
        Direction {x: 0.0, y: 0.0},
        PlayerComponent
    ));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut directions: Query<(&mut Direction, &PlayerComponent)>) {
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

fn spawn_asteroid(time: Res<Time>, mut timer: ResMut<AsteroidSpawTimer>, mut commands: Commands, asset_server: Res<AssetServer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load("meteorGrey_big3.png")),
            Transform::from_xyz(rand::random_range(-200.0..=200.0), 0.0, 0.0),
            Speed(250.0),
            Direction {x: 0.0, y: -1.0}
        ));
    }
}