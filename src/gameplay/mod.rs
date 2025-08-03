use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

use crate::database;

const PLAYER_SPRITE_PATH: &str = "sprites/playerShip1_blue.png";
const ASTEROID_SPRITE_PATH: &str = "sprites/meteorGrey_big3.png";
const LAZER_SPRITE_PATH: &str = "sprites/laserBlue03.png";

pub const PLAYER_SPAWN_HEIGHT: f32 = -400.0;
pub const PLAYER_MOVE_SPEED: f32 = 250.0;
const PLAYER_BODY_SIZE: Vec2 = Vec2::new(34.0, 75.0);
const PLAYER_WINGS_SIZE: Vec2 = Vec2::new(99.0, 35.0);

const LAZER_SPEED: f32 = 600.0;
const LAZER_Y_OFFSET: f32 = 40.0;
const LAZER_LAYER: f32 = -1.0;

const ASTEROID_MOVE_SPEED: f32 = 350.0;
const ASTEROID_SPAWN_HEIGHT: f32 = 550.0;
const ASTEROID_SPAWN_DIAPASON: Vec2 = Vec2::new(-200.0, 200.0);
const ASTEROID_DIAMETER: f32 = 82.0;
const ASTEROID_DAMAGE: i32 = 1;

const SCORE_BY_ONE_ASTEROID: i32 = 5;

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Lazer;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(i32);

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Destroy;

#[derive(Component)]
pub struct DespawnOnRestart;
#[derive(Component)]
pub struct DespawnOnExit;

#[derive(Resource)]
pub struct AsteroidSpawTimer(pub Timer);

#[derive(Resource)]
pub struct LazerShootingTimer(pub Timer);

#[derive(Resource, Deref)]
pub struct LazerShootingSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
pub struct DamageSound(pub Handle<AudioSource>);

#[derive(Event, Default)]
pub struct AsteroidCollisionByLazerEvent;
#[derive(Event, Default)]
pub struct AsteroidDamageCollisionEvent;
#[derive(Event, Default)]
pub struct GameOverEvent;
#[derive(Event, Default)]
pub struct RestartEvent;

#[derive(Resource)]
pub struct Score(pub i32);
#[derive(Resource)]
pub struct ScoreRecord(pub i32);
#[derive(Resource, PartialEq)]
pub enum GameplayState {
    Game,
    GameOver
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Sprite::from_image(asset_server.load(PLAYER_SPRITE_PATH)),
        Transform::from_xyz(0.0, PLAYER_SPAWN_HEIGHT, 0.0),
        Speed(PLAYER_MOVE_SPEED),
        Direction {x: 0.0, y: 0.0},
        Health(3),
        DespawnOnRestart,
        DespawnOnExit,
        Player
    ));
}

pub fn insert_resources(
    mut commands: Commands
) {
    commands.insert_resource(GameplayState::Game);
    commands.insert_resource(AsteroidSpawTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    commands.insert_resource(LazerShootingTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(Score(0));
}

pub fn remove_resources(
    mut commands: Commands
) {
    commands.remove_resource::<GameplayState>();
    commands.remove_resource::<AsteroidSpawTimer>();
    commands.remove_resource::<LazerShootingTimer>();
    commands.remove_resource::<Score>();
}

pub fn cleanup(
    mut commands: Commands,
    despawn_entities: Query<Entity, With<DespawnOnExit>>
) {
    for entity in despawn_entities {
        commands.entity(entity).despawn();
    }
}

pub fn handle_input(
    input: Res<ButtonInput<KeyCode>>, 
    mut directions: Query<&mut Direction, With<Player>>
) {
    for mut dir in &mut directions {
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

pub fn move_objects(
    time: Res<Time>, 
    mut transforms: Query<(&mut Transform, &Direction, &Speed)>
) {
    for (mut transform, dir, speed) in &mut transforms {
        transform.translation.x += dir.x * speed.0 * time.delta_secs();
        transform.translation.y += dir.y * speed.0 * time.delta_secs();
    }
}

pub fn lazer_shooting(
    time: Res<Time>, 
    mut timer: ResMut<LazerShootingTimer>, 
    input: Res<ButtonInput<KeyCode>>, 
    player: Query<&Transform, With<Player>>, 
    sound: Res<LazerShootingSound>,
    asset_server: Res<AssetServer>, 
    mut commands: Commands 
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if input.just_pressed(KeyCode::Space) {
        for player_transform in &player {
            commands.spawn((
                Sprite::from_image(asset_server.load(LAZER_SPRITE_PATH)),
                Transform::from_xyz(player_transform.translation.x, player_transform.translation.y + LAZER_Y_OFFSET, LAZER_LAYER),
                Speed(LAZER_SPEED),
                Direction {x: 0.0, y: 1.0},
                DespawnOnRestart,
                DespawnOnExit,
                Lazer
            ));
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
        timer.0.reset();
    }
}

pub fn spawn_asteroid(
    time: Res<Time>, 
    mut timer: ResMut<AsteroidSpawTimer>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load(ASTEROID_SPRITE_PATH)),
            Transform::from_xyz(rand::random_range(ASTEROID_SPAWN_DIAPASON.x..=ASTEROID_SPAWN_DIAPASON.y), ASTEROID_SPAWN_HEIGHT, 0.0),
            Speed(ASTEROID_MOVE_SPEED),
            Direction {x: 0.0, y: -1.0},
            DespawnOnRestart,
            DespawnOnExit,
            Asteroid
        ));
    }
}

pub fn check_lazer_collision(
    lazers: Query<(Entity, &Transform), (With<Lazer>, Without<Asteroid>)>, 
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Lazer>)>,
    mut collision_events: EventWriter<AsteroidCollisionByLazerEvent>, 
    mut commands: Commands
) {
    for (lazer_entity, lazer) in &lazers {
        if lazer.translation.y > ASTEROID_SPAWN_HEIGHT {
            commands.entity(lazer_entity).insert(Destroy);
            return;
        }
        
        for (asteroid_entity, astreroid) in &asteroids {
            let asteroid_collider =  BoundingCircle::new(astreroid.translation.truncate(), ASTEROID_DIAMETER / 2.0);
            let lazer_collider = Aabb2d::new(lazer.translation.truncate(), lazer.scale.truncate() / 2.0);

            if lazer_collider.intersects(&asteroid_collider) {
                commands.entity(lazer_entity).insert(Destroy);
                commands.entity(asteroid_entity).insert(Destroy);
                collision_events.write_default();
            }
        }
    }
}

pub fn check_player_collision(
    mut player: Single<&mut Transform, (With<Player>, Without<Asteroid>)>,
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Player>)>,
    sound: Res<DamageSound>,
    mut collision_writer: EventWriter<AsteroidDamageCollisionEvent>,
    mut commands: Commands
) { 
    let player_transform = &mut*player;

    if player_transform.translation.x < ASTEROID_SPAWN_DIAPASON.x { player_transform.translation.x = ASTEROID_SPAWN_DIAPASON.x }
    else if player_transform.translation.x > ASTEROID_SPAWN_DIAPASON.y { player_transform.translation.x = ASTEROID_SPAWN_DIAPASON.y }

    let player_center = player_transform.translation.truncate();
    let body_collider = Aabb2d::new(player_center, PLAYER_BODY_SIZE / 2.0);
    let wing_collider = Aabb2d::new(player_center, PLAYER_WINGS_SIZE / 2.0);

    for (asteroid_entity, asteroid_transform) in &asteroids {
        let asteroid_collider = BoundingCircle::new(asteroid_transform.translation.truncate(), ASTEROID_DIAMETER / 2.0);
        if body_collider.intersects(&asteroid_collider) || wing_collider.intersects(&asteroid_collider) {
            collision_writer.write_default();
            commands.entity(asteroid_entity).insert(Destroy);
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }
}

pub fn check_botton_wall_collsion(
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Player>)>,
    sound: Res<DamageSound>,
    mut collision_events: EventWriter<AsteroidDamageCollisionEvent>, 
    mut commands: Commands,
) {
    for (entity, transform) in &asteroids {
        if transform.translation.y < -ASTEROID_SPAWN_HEIGHT {
            collision_events.write(AsteroidDamageCollisionEvent);
            commands.entity(entity).insert(Destroy);
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }
}

pub fn handle_asteroid_damage_collision(
    player_entity: Single<Entity, With<Player>>,
    mut reader: EventReader<AsteroidDamageCollisionEvent>,
    mut commands: Commands
) {
    if !reader.is_empty() {
        let mut damage = 0;
        for _ in reader.read() {
            damage += ASTEROID_DAMAGE;
        }

        commands.entity(player_entity.entity()).insert(Damage(damage));
        reader.clear();
    }
}

pub fn take_damage(
    a: Query<(&mut Health, &Damage, Entity)>,
    mut commands: Commands
) {
    for (mut health, damage, entity) in a {
        health.0 -= damage.0;
        if health.0 <= 0 {
            commands.entity(entity).insert(Dead);
        }
        commands.entity(entity).remove::<Damage>();
    }
}

pub fn handle_player_dead(
    mut game_state: ResMut<GameplayState>,
    mut writer: EventWriter<GameOverEvent>,
    mut player: Single<&mut Sprite, (With<Player>, With<Dead>)>
) {
    if *game_state == GameplayState::GameOver { return; }

    *game_state = GameplayState::GameOver;
    let sprite = &mut*player;
    sprite.color = Color::srgb(1.0, 0.0, 0.0);
    writer.write_default();
}

pub fn handle_game_over_event(
    score_res: Res<Score>,
    mut record_res: ResMut<ScoreRecord>,
    mut state: ResMut<GameplayState>,
    mut event_reader: EventReader<GameOverEvent>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        *state = GameplayState::GameOver;

        let score = score_res.0;
        let mut record = record_res.0;

        if score > record {
            record = score;
            record_res.0 = record;
            database::save_record(score);
        }
    }
}

pub fn calculate_score(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<AsteroidCollisionByLazerEvent>
) {
    if !event_reader.is_empty() {
        for _e in event_reader.read() {
            score.0 += SCORE_BY_ONE_ASTEROID;
        }

        event_reader.clear();
    }
}

pub fn destroy_system(
    destroyed_entities: Query<Entity, With<Destroy>>,
    mut commands: Commands
) {
    for entity in destroyed_entities {
        commands.entity(entity).despawn();
    }
}

pub fn restart_system(
    despawn_entities: Query<Entity, With<DespawnOnRestart>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut state: ResMut<GameplayState>,
    mut event_reader: EventReader<RestartEvent>,
    mut commands: Commands
) {
    if !event_reader.is_empty() {
        event_reader.clear();

        for entity in despawn_entities {
            commands.entity(entity).despawn();
        }
        score.0 = 0;
        *state = GameplayState::Game;
        setup(commands, asset_server);
    }
}