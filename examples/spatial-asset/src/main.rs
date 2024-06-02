use crate::asset::{CustomAsset, CustomAssetLoader};
use crate::camera::{CameraPlugin, FpsCam};
use crate::ui::UiPlugin;
use bevy::asset::LoadState;
use bevy::math::primitives::Plane3d;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_kira_components::kira::sound::Region;
use bevy_kira_components::kira::spatial::emitter::EmitterDistances;
use bevy_kira_components::prelude::*;
use bevy_kira_components::AudioPlugin;

mod asset;
mod camera;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin, CameraPlugin, UiPlugin))
        .init_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_systems(Startup, (setup_custom_asset, setup_scene_basics))
        .add_systems(
            PreUpdate,
            (setup_spatial_sphere_after_load, custom_asset_load_check),
        )
        .add_systems(Update, rotate_objects)
        .run();
}

#[derive(Component)]
struct Rotate(Quat);

fn setup_scene_basics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn(PbrBundle {
        transform: Transform::from_scale(Vec3::splat(100.0)),
        mesh: meshes.add(Plane3d::default().mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::SILVER,
            ..default()
        }),
        ..default()
    });

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::NEG_Y, Vec3::Z),
        ..default()
    });

    // Camera
    commands.spawn((
        AudioListener,
        FpsCam::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 0.).looking_at(vec3(0., 1., -6.), Vec3::Y),
            ..default()
        },
    ));
}

fn rotate_objects(time: Res<Time>, mut q: Query<(&mut Transform, &Rotate)>) {
    let dt = time.delta_seconds();
    if dt < 1e-6 {
        return;
    }
    for (mut transform, rotate) in &mut q {
        let quat = Quat::IDENTITY.slerp(rotate.0, dt);
        transform.rotate(quat);
    }
}

#[derive(Resource)]
pub struct CustomAssetHandle {
    pub handle: Handle<CustomAsset>,
    pub is_loaded: bool,
}

fn setup_custom_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CustomAssetHandle {
        handle: asset_server.load("drums.custom"),
        is_loaded: false,
    });
}

fn custom_asset_load_check(
    asset_server: Res<AssetServer>,
    custom_assets: Res<Assets<CustomAsset>>,
    mut custom_asset_handle: ResMut<CustomAssetHandle>,
) {
    if custom_asset_handle.is_loaded {
        return;
    }

    if asset_server.load_state(&custom_asset_handle.handle) == LoadState::Loaded {
        info!("Custom asset loaded");
        let custom_asset = custom_assets.get(&custom_asset_handle.handle).unwrap();

        if asset_server.load_state(&custom_asset.handle) == LoadState::Loaded {
            info!("Audio file loaded");
            custom_asset_handle.is_loaded = true;
        }
    }
}

fn setup_spatial_sphere_after_load(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    custom_asset_handle: ResMut<CustomAssetHandle>,
    custom_assets: Res<Assets<CustomAsset>>,
    mut setup: Local<bool>,
) {
    if custom_asset_handle.is_loaded && !*setup {
        *setup = true;

        let custom_asset = custom_assets.get(&custom_asset_handle.handle).unwrap();

        // Passing a direct source to the emitter works as well, but only if it
        // is a different asset path to the one embedded in custom asset. Try
        // changing this to just drums.ogg and notice that spatialization does
        // not work either. I.e. it seems that when something else "owns" the
        // handle spatialization does not work, but if `source` gets its own
        // "unique" handle, it works.
        // let direct_source = asset_server.load("drums_direct.ogg");

        commands
            .spawn((
                Rotate(Quat::from_rotation_y(1.0)),
                InheritedVisibility::VISIBLE,
                TransformBundle {
                    local: Transform::from_xyz(0., 1., -6.0),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn((
                    SpatialEmitter {
                        distances: EmitterDistances {
                            min_distance: 3.0,
                            max_distance: 7.0,
                        },
                        ..default()
                    },
                    AudioFileBundle {
                        source: custom_asset.handle.clone(),
                        // source: direct_source,
                        settings: AudioFileSettings {
                            loop_region: Some(Region::from(3.6..6.0)),
                            ..default()
                        },
                        ..default()
                    },
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.1).mesh()),
                        material: materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            emissive: Color::GREEN,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0., 0., 2.5),
                        ..default()
                    },
                ));
            });
    }
}
