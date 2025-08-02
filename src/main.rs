use bevy::prelude::*;

mod gameplay;
mod ui;
mod database;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(gameplay::GameplayState::Game)
        .insert_resource(gameplay::AsteroidSpawTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(gameplay::LazerShootingTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .insert_resource(gameplay::Score(0))
        .add_event::<gameplay::AsteroidCollisionByLazerEvent>()
        .add_event::<gameplay::AsteroidDamageCollisionEvent>()
        .add_event::<gameplay::GameOverEvent>()

        .init_state::<GameState>()
        .add_systems(Startup, (startup, load_audio))
        .add_systems(Update, ui::button_system)
        
        .add_systems(OnEnter(GameState::MainMenu), ui::setup_menu)
        .add_systems(Update, (ui::main_menu_action).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), ui::cleanup_menu)

        .add_systems(OnEnter(GameState::InGame), ui::setup_hud)
        .add_systems(Update, (
            gameplay::handle_input, 
            gameplay::lazer_shooting, 
            gameplay::spawn_asteroid, 
            gameplay::move_objects, 
            gameplay::check_lazer_collision, 
            gameplay::check_player_collision, 
            gameplay::check_botton_wall_collsion,
            gameplay::handle_asteroid_damage_collision,
            gameplay::take_damage,
            gameplay::handle_player_dead,
            gameplay::handle_game_over_event,
            gameplay::destroy_system,
            gameplay::calculate_score,
            ui::update_player_health_ui,
            ui::update_score_ui
        ).run_if(in_state(GameState::InGame)).chain())
        .add_systems(OnExit(GameState::InGame), ui::cleanup_hud)

        .run();
}

fn startup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);
}

fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let lazer_shooting_sound = asset_server.load("audio/sfx_laser1.ogg");
    commands.insert_resource(gameplay::LazerShootingSound(lazer_shooting_sound));

    let damage_sound = asset_server.load("audio/sfx_lose.ogg");
    commands.insert_resource(gameplay::DamageSound(damage_sound));
}
