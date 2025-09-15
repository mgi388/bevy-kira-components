use bevy::color::palettes::css::{GRAY, GREEN};
use bevy::prelude::*;

use bevy_kira_components::kira::sound::Region;
use bevy_kira_components::kira::tween::Tween;
use bevy_kira_components::prelude::AudioBundle;
use bevy_kira_components::prelude::*;
use bevy_kira_components::sources::audio_file::AudioFileEndBehavior;
use diagnostics_ui::DiagnosticsUiPlugin;

use crate::ui::UiPlugin;

mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin, DiagnosticsUiPlugin, UiPlugin))
        .add_systems(Startup, init)
        .add_systems(Update, handle_interactive_sound)
        .run();
}

#[derive(Component)]
struct InteractiveSound;

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let audio_file = asset_server.load::<AudioFile>("drums.ogg");
    commands.spawn((
        Sprite {
            color: GRAY.into(),
            ..default()
        },
        Transform::from_scale(Vec3::splat(25.0)),
        AudioBundle::new(AudioSourceHandle(audio_file)).with_settings(AudioFileSettings {
            loop_region: Some(Region::from(3.6..6.0)),
            start_paused: true,
            ..default()
        }),
        InteractiveSound,
    ));
}

fn handle_interactive_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut AudioHandle<AudioFileHandle>, &mut Sprite), With<InteractiveSound>>,
) {
    if keyboard.just_pressed(KeyCode::KeyA) {
        commands.spawn((
            AudioFileBundle::new(AudioSourceHandle(asset_server.load("click.wav"))),
            AudioFileEndBehavior::Despawn,
        ));
    }
    if keyboard.just_pressed(KeyCode::Space) {
        for (mut handle, mut sprite) in &mut q {
            handle.resume(Tween::default());
            sprite.color = GREEN.into();
        }
    } else if keyboard.just_released(KeyCode::Space) {
        for (mut handle, mut sprite) in &mut q {
            handle.pause(Tween::default());
            sprite.color = GRAY.into();
        }
    }
}
