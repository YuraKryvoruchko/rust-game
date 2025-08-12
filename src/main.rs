use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::audio::Volume;

mod ui;
mod audio;
use crate::{audio::*, gameplay::{GameplayState, ScoreRecord}, ui::{MenuState, MusicVolumeText, ScoreRecordText, SoundVolumeText}};

mod gameplay;
mod database;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(audio::MusicVolume(100.0))
        .insert_resource(audio::SoundVolume(100.0))
        .insert_resource(gameplay::ScoreRecord(database::get_record()))
        .add_event::<gameplay::AsteroidCollisionByLazerEvent>()
        .add_event::<gameplay::AsteroidDamageCollisionEvent>()
        .add_event::<gameplay::GameOverEvent>()
        .add_event::<gameplay::RestartEvent>()

        .init_state::<GameState>()
        .init_state::<MenuState>()
        .init_state::<GameplayState>()

        .add_systems(Startup, (startup, load_audio, setup_background_music))
        .add_systems(Update, (
            ui::button_system, 
            ui::slider_system,
            audio::volume_system::<Music, MusicVolume>.run_if(resource_changed::<MusicVolume>),
            audio::volume_system::<Sound, SoundVolume>.run_if(resource_changed::<SoundVolume>)
        ))
        
        .add_systems(OnEnter(GameState::MainMenu), ui::setup_menu)
        .add_systems(OnEnter(MenuState::MainMenu), ui::setup_main_menu)
        .add_systems(OnExit(MenuState::MainMenu), ui::cleanup_main_menu)
        .add_systems(OnEnter(MenuState::Settings), ui::setup_settings_menu)
        .add_systems(OnExit(MenuState::Settings), ui::cleanup_settings_menu)
        .add_systems(Update, (
            ui::menu_button_action, 
            ui::menu_slider_action, 
            ui::resource_value_text::<MusicVolumeText, MusicVolume>,
            ui::resource_value_text::<SoundVolumeText, SoundVolume>,
            ui::resource_value_text::<ScoreRecordText, ScoreRecord>
        ).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), ui::cleanup_main_menu)

        .add_systems(OnEnter(GameState::InGame), (gameplay::insert_resources, gameplay::setup, ui::setup_hud))
        .add_systems(OnEnter(GameplayState::Game), gameplay::setup_gameplay)
        .add_systems(Update, (
            gameplay::handle_input, 
            gameplay::lazer_shooting, 
            gameplay::spawn_asteroid, 
            gameplay::move_objects, 
            gameplay::check_lazer_collision, 
            gameplay::check_player_collision, 
            gameplay::check_botton_wall_collsion,
            gameplay::handle_asteroid_damage_collision,
            gameplay::handle_player_damage,
            gameplay::take_damage,
            gameplay::handle_player_dead,
            gameplay::destroy_system,
            gameplay::calculate_score,
            gameplay::rotate_around,
            gameplay::flick_sprites
        ).run_if(in_state(GameplayState::Game)).chain())
        .add_systems(OnEnter(GameplayState::GameOver), (gameplay::handle_game_over_event, ui::handle_game_over))
        .add_systems(Update, (
            gameplay::restart_system,
            ui::game_over_panel_action,
        ).run_if(in_state(GameplayState::GameOver)).chain())
        .add_systems(Update, (
            ui::update_player_health_ui,
            ui::update_score_ui,
            ui::update_ui_padding,
        ).run_if(in_state(GameState::InGame)).chain())
        .add_systems(OnExit(GameState::InGame), (gameplay::cleanup, gameplay::remove_resources, ui::cleanup_hud))

        .run();
}

fn startup(
    mut commands: Commands
) {
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical { viewport_height: 1080.0 },
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((projection, Camera2d));
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

fn setup_background_music(
    asset_server: Res<AssetServer>,
    music_volume: Res<MusicVolume>,
    mut commands: Commands
) {
    println!("setup_background_music");
    let background_music: Handle<AudioSource> = asset_server.load("audio/639495__romariogrande__space-ambient-voyage.ogg");
    commands.spawn((AudioPlayer(background_music.clone()), Music, PlaybackSettings {volume: Volume::Linear(music_volume.0 / 100.0), ..PlaybackSettings::LOOP }));
}
