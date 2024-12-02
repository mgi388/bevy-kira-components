use bevy::color::palettes::css::{GRAY, GREEN};
use bevy::math::vec3;
use bevy::prelude::*;

use bevy_kira_components::kira::sound::{PlaybackRate, Region};
use bevy_kira_components::kira::tween::Tween;
use bevy_kira_components::prelude::*;
use bevy_kira_components::AudioPlugin;
use diagnostics_ui::DiagnosticsUiPlugin;

use crate::camera::{CameraPlugin, FpsCam};
use crate::motion::{Motion, MotionPlugin};
use crate::ui::UiPlugin;

mod camera;
mod motion;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            DiagnosticsUiPlugin,
            MotionPlugin,
            CameraPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, (init_camera, init_objects))
        .add_systems(Update, (rotate_objects, fake_doppler_effect))
        .run();
}

fn init_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0., 2., 0.).looking_at(vec3(0., 1., -6.), Vec3::Y);
    commands.spawn((
        AudioListener,
        FpsCam::default(),
        Camera3d::default(),
        transform,
    ));
}

#[derive(Component)]
struct Rotate(Quat);

fn init_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let source = asset_server.load("drums.ogg");
    // Audio emitter
    commands
        .spawn((
            Rotate(Quat::from_rotation_y(1.0)),
            InheritedVisibility::VISIBLE,
            Transform::from_xyz(0., 1., -6.0),
        ))
        .with_children(|children| {
            children.spawn((
                Doppler(1.0),
                SpatialEmitter::default(),
                AudioFileBundle {
                    source: AudioSourceHandle(source),
                    settings: AudioFileSettings {
                        loop_region: Some(Region::from(3.6..6.0)),
                        ..default()
                    },
                    ..default()
                },
                Transform::from_xyz(0., 0., 2.5),
                Mesh3d(meshes.add(Sphere::new(0.1).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: GREEN.into(),
                    ..default()
                })),
            ));
        });

    // Plane
    commands.spawn((
        Transform::from_scale(Vec3::splat(100.0)),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::ONE).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRAY.into(),
            ..default()
        })),
    ));

    // Sun
    commands.spawn((
        Transform::default().looking_at(Vec3::NEG_Y, Vec3::Z),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
    ));
}

fn rotate_objects(time: Res<Time>, mut q: Query<(&mut Transform, &Rotate)>) {
    let dt = time.delta_secs();
    if dt < 1e-6 {
        return;
    }
    for (mut transform, rotate) in &mut q {
        let quat = Quat::IDENTITY.slerp(rotate.0, dt);
        transform.rotate(quat);
    }
}

/// Stores the computed doppler value to display in UI
#[derive(Component)]
struct Doppler(f32);

/// Marker component for the UI
#[derive(Component)]
struct DopplerUI;

const SPEED_OF_SOUND: f32 = 20.0;

// Fake the doppler pitch shift effect by playing the loop faster or slower
fn fake_doppler_effect(
    mut q: Query<
        (
            &mut AudioHandle<AudioFileHandle>,
            &mut Doppler,
            &GlobalTransform,
            &Motion,
        ),
        With<SpatialEmitter>,
    >,
    q_cameras: Query<(&GlobalTransform, &Motion), With<FpsCam>>,
) {
    let Ok((cam_transform, cam_motion)) = q_cameras.get_single() else {
        // Motion has not been added yet, wait one tick
        return;
    };
    let cam_transform = cam_transform.compute_transform();
    for (mut handle, mut doppler, transform, motion) in &mut q {
        let local_dir = Vec3::normalize(cam_transform.translation - transform.translation());
        doppler.0 = (SPEED_OF_SOUND - cam_motion.motion().dot(local_dir))
            / (SPEED_OF_SOUND - motion.motion().dot(local_dir));
        handle.set_playback_rate(PlaybackRate::Factor(doppler.0 as _), Tween::default());
    }
}
