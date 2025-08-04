use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_ecs::observer::TriggerTargets;
use bevy_ecs::relationship::RelatedSpawnerCommands;

use crate::gameplay::*;
use crate::GameState;

mod slider;
use slider::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const DEFAULT_MARGIN: UiRect = UiRect::all(Val::Px(5.0));

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameOverPanel;

#[derive(Component)]
pub enum GameOverPanelButtonAction {
    Restart,
    ExitToMenu,
}

pub fn setup_hud(
    mut commands: Commands
) {
    commands.spawn((
        Hud,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
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
            HealthText
        ));

        parent.spawn((
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
    });
}

pub fn cleanup_hud(
    hud: Single<Entity, With<Hud>>,
    mut commands: Commands
) {
    commands.entity(hud.entity()).despawn();
}

pub fn game_over_panel_action(
    interaction_query: Query<
        (&Interaction, &GameOverPanelButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut game_over_writer: EventWriter<RestartEvent>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, action) in interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                GameOverPanelButtonAction::Restart => {
                    game_over_writer.write_default();
                }
                GameOverPanelButtonAction::ExitToMenu => {
                    game_state.set(GameState::MainMenu);
                }
            }
        }
    }
}

pub fn update_player_health_ui(
    health: Single<&Health, With<Player>>,
    mut text_query: Query<&mut TextSpan, With<HealthText>>
) {
    let value = health.0;
    for mut span in &mut text_query {
        **span = format!("{value}");
    }
}

pub fn update_score_ui(
    score: Res<Score>,
    mut text_query: Query<&mut TextSpan, With<ScoreText>>
) {
    let value = score.0;
    for mut span in &mut text_query {
        **span = format!("{value}")
    }
}

pub fn handle_game_over(
    current_score: Res<Score>,
    record_score: Res<ScoreRecord>,
    commands: Commands,
    mut event_reader: EventReader<GameOverEvent>
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        spawn_game_over_panel(current_score.0, record_score.0, commands)
    }
}

fn spawn_game_over_panel(
    current_score: i32,
    record_score: i32,
    mut commands: Commands
) {
    commands.spawn((
        Node {
            width: Val::Percent(60.0),
            height: Val::Percent(60.0),
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        Visibility::Visible,
        DespawnOnRestart,
        DespawnOnExit,
        GameOverPanel
    ))
    .with_children(|parent| {
        create_text(parent, 40.0, "GAME OVER!");
        create_text(parent, 20.0, &format!("Score: {current_score}"));
        create_text(parent, 20.0, &format!("Your record: {record_score}"));
        
        parent.spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            create_button(parent, 170.0, 70.0, "Restart", GameOverPanelButtonAction::Restart);
            create_button(parent, 170.0, 70.0, "Exit", GameOverPanelButtonAction::ExitToMenu);
        });
    });
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Reset,
    Exit,
}

pub fn setup_menu(
    mut commands: Commands
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        Visibility::Visible,
        MainMenu
    ))
    .with_children(|parent| {
        create_text(parent, 50.0, "Rust-Shooter");
        create_button(parent, 300.0, 90.0, "Play", MenuButtonAction::Play);
        create_button(parent, 300.0, 90.0, "Settings", MenuButtonAction::Settings);
        create_button(parent, 300.0, 90.0, "Reset record", MenuButtonAction::Reset);
        create_button(parent, 300.0, 90.0, "Exit", MenuButtonAction::Exit);
        create_slider(parent);
        create_slider(parent);
    }); 
}

pub fn cleanup_menu(
    menu: Single<Entity, With<MainMenu>>,
    mut commands: Commands
) {
    commands.entity(menu.entity()).despawn();
}

pub fn main_menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, action) in interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Exit => {
                    app_exit_events.write_default();
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::InGame);
                }
                _ => ()
            }
        }
    }
}

pub fn create_slider(
    commands: &mut RelatedSpawnerCommands<'_, ChildOf>
) {
    commands.spawn((
        Node {
            width: Val::Px(100.0),
            height: Val::Px(50.0),
            ..Default::default()
        },
        BackgroundColor(Color::BLACK)
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(5.0)),
                ..Default::default()  
            },
            Interaction::default(),
            RelativeCursorPosition::default(),
            Slider { min: 10.0, max: 30.0, value: 0.0 }
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
                SliderBar
            ));
        });
    });
}

pub fn button_system(
    interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor
    ),
    (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut background_color) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn slider_system(
    sliders: Query<(&RelativeCursorPosition, &Interaction, &Children, &mut Slider)>,
    mut slider_bars: Query<&mut Node, With<SliderBar>>
) {
    for (pos, interaction, children, mut slider) in sliders {
        if *interaction == Interaction::Pressed {
            match pos.normalized {
                Some(vec) => {
                    slider.set_value(vec.x);
                },
                None => ()
            }

            for child in children.entities() {
                let node = slider_bars.get_mut(child);
                match node {
                    Result::Err(err) => {
                        println!("Warning: {}", err);
                    }
                    Result::Ok(mut node) => {
                        node.width = Val::Percent(slider.value * 100.0);
                    }
                }
            }
        }
    }
}

fn create_text(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>, 
    size: f32, 
    text: &str
) {
    parent.spawn((
        Node {
            margin: DEFAULT_MARGIN,
            ..Default::default()
        },
        Text::new(text),
        TextFont {
            font_size: size,
            ..Default::default()
        }
    ));
}

fn create_button<A: Component>(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    width: f32,
    height: f32, 
    button_text: &str,
    button_action: A
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            margin: DEFAULT_MARGIN,
            ..Default::default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        button_action,
        children![(
            Text::new(button_text),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default()
        )],
    ));
}
