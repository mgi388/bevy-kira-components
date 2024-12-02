//! Audio source gizmos.

use bevy::{color::palettes::tailwind::*, prelude::*, transform::TransformSystem};

use crate::spatial::SpatialEmitter;

fn spatial_emitter_gizmo(
    transform: &GlobalTransform,
    spatial_emitter: &SpatialEmitter,
    inner_color: Color,
    outer_color: Color,
    gizmos: &mut Gizmos<AudioSourceGizmoConfigGroup>,
) {
    gizmos
        .sphere(
            transform.to_isometry(),
            spatial_emitter.distances.min_distance,
            inner_color,
        )
        .resolution(32);
    gizmos
        .sphere(
            transform.to_isometry(),
            spatial_emitter.distances.max_distance,
            outer_color,
        )
        .resolution(32);

    gizmos.primitive_3d(&CONICAL_FRUSTUM, transform.to_isometry(), outer_color);
}

const CONICAL_FRUSTUM: ConicalFrustum = ConicalFrustum {
    radius_top: BIG_3D,
    radius_bottom: SMALL_3D,
    height: BIG_3D,
};

const SMALL_3D: f32 = 0.5;
const BIG_3D: f32 = 1.0;

/// A plugin that adds gizmos for audio sources.
pub struct AudioSourceGizmoPlugin;

impl Plugin for AudioSourceGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AudioSourceGizmoConfigGroup>()
            .init_gizmo_group::<AudioSourceGizmoConfigGroup>()
            .add_systems(
                PostUpdate,
                (
                    draw_all_audio_sources.run_if(|config: Res<GizmoConfigStore>| {
                        config.config::<AudioSourceGizmoConfigGroup>().1.draw_all
                    }),
                )
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

/// The color of the audio source gizmo.
#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum AudioSourceGizmoColor {
    /// Use the default color.
    #[default]
    Default,
}

/// Configuration for the audio source gizmo.
#[derive(Clone, Reflect, GizmoConfigGroup)]
pub struct AudioSourceGizmoConfigGroup {
    /// Draw all audio sources, regardless of their settings.
    pub draw_all: bool,
    /// The color of the audio source gizmo.
    pub color: AudioSourceGizmoColor,
    /// The color of the inner sphere of the spatial emitter gizmo.
    pub spatial_emitter_inner_color: Color,
    /// The color of the outer sphere of the spatial emitter gizmo.
    pub spatial_emitter_outer_color: Color,
}

impl Default for AudioSourceGizmoConfigGroup {
    fn default() -> Self {
        Self {
            draw_all: false,
            color: AudioSourceGizmoColor::Default,
            spatial_emitter_inner_color: RED_400.into(),
            spatial_emitter_outer_color: RED_100.into(),
        }
    }
}

fn draw_all_audio_sources(
    spatial_emitter_query: Query<(Entity, &SpatialEmitter, &GlobalTransform)>,
    mut gizmos: Gizmos<AudioSourceGizmoConfigGroup>,
) {
    match gizmos.config_ext.color {
        AudioSourceGizmoColor::Default => {
            for (_, spatial_emitter, transform) in &spatial_emitter_query {
                spatial_emitter_gizmo(
                    transform,
                    spatial_emitter,
                    gizmos.config_ext.spatial_emitter_inner_color,
                    gizmos.config_ext.spatial_emitter_outer_color,
                    &mut gizmos,
                );
            }
        }
    }
}
