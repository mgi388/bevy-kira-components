use crate::{Doppler, DopplerUI};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_init)
            .add_systems(Update, update_ui_doppler);
    }
}

fn ui_init(mut commands: Commands) {
    let text_font = TextFont {
        font_size: 14.0,
        ..default()
    };
    commands
        .spawn(Node {
            margin: UiRect::all(Val::Px(8.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|children| {
            let mut child = |text: &str| {
                children.spawn((Text::new(text), text_font.clone()));
            };

            child("Click on window to lock mouse and allow movement");
            child("Use WASD or arrows to move");
            child("Use mouse to look");
            child("Press Escape to release mouse");
            children
                .spawn((DopplerUI, Text::default()))
                .with_children(|p| {
                    p.spawn((TextSpan::new("Doppler factor "), text_font.clone()));
                    p.spawn((TextSpan::new("1.00x"), text_font.clone()));
                });
        });
}

fn update_ui_doppler(
    mut writer: TextUiWriter,
    q_text: Query<Entity, With<DopplerUI>>,
    q_doppler: Query<&Doppler>,
) {
    let text_entity = q_text.single();
    let doppler = q_doppler.single().0;
    *writer.text(text_entity, 2) = format!("{doppler:1.2}x");
}
