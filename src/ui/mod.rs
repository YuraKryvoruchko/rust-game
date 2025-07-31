use bevy::prelude::*;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameOverPanel;

pub fn load_ui(
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
        HealthText
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
