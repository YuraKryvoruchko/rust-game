use bevy::prelude::*;
use core::fmt::Display;

mod ui;
use crate::{gameplay::ScoreRecord, ui::{MenuState, ScoreRecordText, VolumeText}};

mod gameplay;
mod database;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Resource)]
pub struct Volume(pub f32);

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Volume(100.0))
        .insert_resource(gameplay::ScoreRecord(database::get_record()))
        .add_event::<gameplay::AsteroidCollisionByLazerEvent>()
        .add_event::<gameplay::AsteroidDamageCollisionEvent>()
        .add_event::<gameplay::GameOverEvent>()
        .add_event::<gameplay::RestartEvent>()

        .init_state::<GameState>()
        .init_state::<MenuState>()
        .add_systems(Startup, (startup, load_audio))
        .add_systems(Update, (ui::button_system, ui::slider_system))
        
        .add_systems(OnEnter(GameState::MainMenu), ui::setup_menu)
        .add_systems(OnEnter(MenuState::MainMenu), ui::setup_main_menu)
        .add_systems(OnExit(MenuState::MainMenu), ui::cleanup_main_menu)
        .add_systems(OnEnter(MenuState::Settings), ui::setup_settings_menu)
        .add_systems(OnExit(MenuState::Settings), ui::cleanup_settings_menu)
        .add_systems(Update, (
            ui::menu_button_action, 
            ui::menu_slider_action, 
            ui::resource_value_text::<VolumeText, Volume>,
            ui::resource_value_text::<ScoreRecordText, ScoreRecord>
        ).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), ui::cleanup_main_menu)

        .add_systems(OnEnter(GameState::InGame), (gameplay::insert_resources, gameplay::setup, ui::setup_hud))
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
            ui::update_score_ui,
            ui::handle_game_over,
            ui::game_over_panel_action,
            gameplay::restart_system
        ).run_if(in_state(GameState::InGame)).chain())
        .add_systems(OnExit(GameState::InGame), (gameplay::cleanup, gameplay::remove_resources, ui::cleanup_hud))

        .run();
}

fn startup(
    mut commands: Commands
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
