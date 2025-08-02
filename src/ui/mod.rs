use bevy::prelude::*;
use bevy_ecs::relationship::RelatedSpawnerCommands;

use crate::GameState;
use crate::gameplay::*;

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

pub fn spawn_game_over_panel(
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
        GameOverPanel
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER!"),
            TextFont {
                font_size: 40.0,
                ..Default::default()
            }
        ));
        parent.spawn(
            Text::new(format!("Score: {current_score}"))
        );
        parent.spawn(
            Text::new(format!("Your record: {record_score}"))
        );
        
        
        // restart button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor(Color::srgb(0.0, 1.0, 0.0)),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Restart"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default()
            ));
        });

        // exit to menu button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor(Color::srgb(0.0, 1.0, 0.0)),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Exit to menu"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default()
            ));
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
    }); 
}

pub fn cleanup_menu(
    menu: Single<Entity, With<MainMenu>>,
    mut commands: Commands
) {
    commands.entity(menu.entity()).despawn();
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
