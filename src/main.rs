use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

const PLAYER_SPAWN_HEIGHT: f32 = -400.0;
const PLAYER_MOVE_SPEED: f32 = 250.0;
const PLAYER_BODY_SIZE: Vec2 = Vec2::new(34.0, 75.0);
const PLAYER_WINGS_SIZE: Vec2 = Vec2::new(99.0, 35.0);

const LAZER_SPEED: f32 = 400.0;
const LAZER_Y_OFFSET: f32 = 40.0;
const LAZER_LAYER: f32 = -1.0;

const ASTEROID_MOVE_SPEED: f32 = 250.0;
const ASTEROID_SPAWN_HEIGHT: f32 = 550.0;
const ASTEROID_SPAWN_DIAPASON: Vec2 = Vec2::new(-200.0, 200.0);
const ASTEROID_DIAMETER: f32 = 82.0;
const ASTEROID_DAMAGE: i32 = 1;

const SCORE_BY_ONE_ASTEROID: i32 = 5;

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
struct Asteroid;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Health(i32);

#[derive(Resource)]
struct AsteroidSpawTimer(Timer);

#[derive(Resource)]
struct LazerShootingTimer(Timer);

#[derive(Resource, Deref)]
struct LazerShootingSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct DamageSound(Handle<AudioSource>);

#[derive(Event)]
struct AsteroidCollisionEvent(Entity);

#[derive(Event)]
struct AsteroidDamageCollisionEvent();

#[derive(Component)]
struct FPSText;
#[derive(Component)]
struct ScoreText;
#[derive(Resource)]
struct Score(i32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AsteroidSpawTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(LazerShootingTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .insert_resource(Score(0))
        .add_event::<AsteroidCollisionEvent>()
        .add_event::<AsteroidDamageCollisionEvent>()
        .add_systems(Startup, (startup, load_audio, load_ui))
        .add_systems(Update, (
            handle_input, 
            lazer_shooting, 
            spawn_asteroid, 
            move_objects, 
            check_lazer_collision, 
            check_player_collision, 
            destroy_asteroids,
            check_botton_wall_collsion,
            handle_damage_asteroid_collision,
            take_damage,
            handle_player_dead,
            update_player_health_ui,
            update_score_ui
        ).chain())
        .run();
}

fn startup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);

    let lazer_shooting_sound = asset_server.load("audio/sfx_laser1.ogg");
    commands.insert_resource(LazerShootingSound(lazer_shooting_sound));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/playerShip1_blue.png")),
        Transform::from_xyz(0.0, PLAYER_SPAWN_HEIGHT, 0.0),
        Speed(PLAYER_MOVE_SPEED),
        Direction {x: 0.0, y: 0.0},
        Health(3),
        Player
    ));
}

fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let lazer_shooting_sound = asset_server.load("audio/sfx_laser1.ogg");
    commands.insert_resource(LazerShootingSound(lazer_shooting_sound));

    let damage_sound = asset_server.load("audio/sfx_lose.ogg");
    commands.insert_resource(DamageSound(damage_sound));
}

fn load_ui(
    mut commands: Commands
) {
    commands.spawn((
        Text::new("Health: "),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..default()
        }
    ))
    .with_child((
        TextSpan::default(),
        FPSText
    ));

    commands.spawn((
        Text::new("Score: "),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.0),
            top: Val::Px(5.0),
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        ScoreText
    ));
}

fn update_player_health_ui(
    health: Single<&Health, With<Player>>,
    mut text_query: Query<&mut TextSpan, With<FPSText>>
) {
    let value = health.0;
    for mut span in &mut text_query {
        **span = format!("{value}");
    }
}

fn update_score_ui(
    score: Res<Score>,
    mut text_query: Query<&mut TextSpan, With<ScoreText>>
) {
    let value = score.0;
    for mut span in &mut text_query {
        **span = format!("{value}")
    }
}

fn handle_input(
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

fn move_objects(
    time: Res<Time>, 
    mut transforms: Query<(&mut Transform, &Direction, &Speed)>
) {
    for (mut transform, dir, speed) in &mut transforms {
        transform.translation.x += dir.x * speed.0 * time.delta_secs();
        transform.translation.y += dir.y * speed.0 * time.delta_secs();
    }
}

fn lazer_shooting(
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
                Sprite::from_image(asset_server.load("sprites/laserBlue03.png")),
                Transform::from_xyz(player_transform.translation.x, player_transform.translation.y + LAZER_Y_OFFSET, LAZER_LAYER),
                Speed(LAZER_SPEED),
                Direction {x: 0.0, y: 1.0},
                Lazer
            ));
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
        timer.0.reset();
    }
}

fn spawn_asteroid(
    time: Res<Time>, 
    mut timer: ResMut<AsteroidSpawTimer>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/meteorGrey_big3.png")),
            Transform::from_xyz(rand::random_range(ASTEROID_SPAWN_DIAPASON.x..=ASTEROID_SPAWN_DIAPASON.y), ASTEROID_SPAWN_HEIGHT, 0.0),
            Speed(ASTEROID_MOVE_SPEED),
            Direction {x: 0.0, y: -1.0},
            Asteroid
        ));
    }
}

fn check_lazer_collision(
    lazers: Query<(Entity, &Transform), (With<Lazer>, Without<Asteroid>)>, 
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Lazer>)>,
    mut collision_events: EventWriter<AsteroidCollisionEvent>, 
    mut commands: Commands
) {
    for (lazer_entity, lazer) in &lazers {
        if lazer.translation.y > ASTEROID_SPAWN_HEIGHT {
            commands.entity(lazer_entity).despawn();
            return;
        }

        for (asteroid_entity, astreroid) in &asteroids {
            let asteroid_collider =  BoundingCircle::new(astreroid.translation.truncate(), ASTEROID_DIAMETER / 2.0);
            let lazer_collider = Aabb2d::new(lazer.translation.truncate(), lazer.scale.truncate() / 2.0);

            if lazer_collider.intersects(&asteroid_collider) {
                commands.entity(lazer_entity).despawn();
                collision_events.write(AsteroidCollisionEvent(asteroid_entity));
            }
        }
    }
}

fn destroy_asteroids(
    mut commands: Commands,
    mut collision_events: EventReader<AsteroidCollisionEvent>,
    mut score: ResMut<Score>
) {
    for event in collision_events.read() {
        commands.entity(event.0).despawn();
        score.0 += SCORE_BY_ONE_ASTEROID;
    }
}

fn check_player_collision(
    mut player: Single<(&mut Transform, &mut Health), (With<Player>, Without<Asteroid>)>,
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Player>)>,
    sound: Res<DamageSound>, 
    mut commands: Commands
) { 
    let (player_transform,  player_health) = &mut*player;

    if player_transform.translation.x < ASTEROID_SPAWN_DIAPASON.x { player_transform.translation.x = ASTEROID_SPAWN_DIAPASON.x }
    else if player_transform.translation.x > ASTEROID_SPAWN_DIAPASON.y { player_transform.translation.x = ASTEROID_SPAWN_DIAPASON.y }

    let player_center = player_transform.translation.truncate();
    let body_collider = Aabb2d::new(player_center, PLAYER_BODY_SIZE / 2.0);
    let wing_collider = Aabb2d::new(player_center, PLAYER_WINGS_SIZE / 2.0);

    for (asteroid_entity, asteroid_transform) in &asteroids {
        let asteroid_collider = BoundingCircle::new(asteroid_transform.translation.truncate(), ASTEROID_DIAMETER / 2.0);
        if body_collider.intersects(&asteroid_collider) || wing_collider.intersects(&asteroid_collider) {
            player_health.0 -= 1;
            println!("Player health: {}", player_health.0);
            commands.entity(asteroid_entity).despawn();
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }
}

fn check_botton_wall_collsion(
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Player>)>,
    sound: Res<DamageSound>,
    mut collision_events: EventWriter<AsteroidCollisionEvent>, 
    mut commands: Commands,
) {
    for (entity, transform) in &asteroids {
        if transform.translation.y < -550.0 {
            collision_events.write(AsteroidCollisionEvent(entity));
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }
}

fn handle_damage_asteroid_collision(
    mut player: Single<(&mut Health, Entity), With<Player>>,
    mut collision_events: EventReader<AsteroidDamageCollisionEvent>,
    mut commands: Commands
) {
    let (health, entity) = &mut* player;
    for _ in collision_events.read() {
        health.0 -= ASTEROID_DAMAGE;
        if health.0 <= 0 {
            commands.entity(*entity).despawn();
        }
    }

    collision_events.clear();
}

#[derive(Component)]
struct Damage(i32);

#[derive(Component)]
struct Dead;

fn take_damage(
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

fn handle_player_dead(
    mut player: Single<(&mut Sprite), (With<Player>, With<Dead>)>
) {
    let sprite = &mut*player;
    sprite.color = Color::srgb(1.0, 0.0, 0.0)
}