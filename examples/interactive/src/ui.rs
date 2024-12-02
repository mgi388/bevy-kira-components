use bevy::app::{App, Plugin};
use bevy::prelude::*;

use bevy_kira_components::sources::audio_file::source::AudioFileHandle;
use bevy_kira_components::sources::AudioHandle;

use crate::InteractiveSound;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_init)
            .add_systems(Update, ui_update);
    }
}

fn ui_init(mut commands: Commands) {
    commands
        .spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(8.)),
                ..default()
        })
        .with_children(|children| {
            let text_font = TextFont {
                font_size: 14.0,
                ..default()
            };
            children.spawn((
                Text::new("Hold Space to play a looping sound, press A (on QWERTY keyboards) to send a one-shot sound"),
                    text_font.clone(),
            ));
            children
                .spawn((
                    PlaybackPos,
                    Text::default(),
                    text_font.clone(),
                ))
                .with_children(|p| {
                    p.spawn((
                        TextSpan::new("Playback position: "),
                        text_font.clone(),
                    ));
                    p.spawn((
                        TextSpan::new("0.0 s"),
                        text_font.clone(),
                    ));

                });
            });
}

#[derive(Component)]
struct PlaybackPos;

fn ui_update(
    mut writer: TextUiWriter,
    q_ui: Query<Entity, With<PlaybackPos>>,
    q_audio: Query<&AudioHandle<AudioFileHandle>, With<InteractiveSound>>,
) {
    let text_entity = q_ui.single();
    let audio_handle_result = q_audio.get_single();
    if let Ok(handle) = audio_handle_result {
        let pos = handle.position();
        *writer.text(text_entity, 1) = format!("{pos:2.1} s");
    }
}
