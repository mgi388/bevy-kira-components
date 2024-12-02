use bevy::diagnostic::{DiagnosticPath, DiagnosticsStore};
use bevy::prelude::*;

pub struct DiagnosticsUiPlugin;

impl Plugin for DiagnosticsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct DiagnosticSource(DiagnosticPath);

fn init_ui(mut commands: Commands, diagnostics: Res<DiagnosticsStore>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.0),
                top: Val::Percent(1.0),
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.4)),
            GlobalZIndex(i32::MAX),
        ))
        .with_children(|children| {
            for diag in diagnostics.iter() {
                let text_font = TextFont {
                    font_size: 16.0,
                    ..default()
                };
                let text_color = TextColor(Color::WHITE);
                children
                    .spawn((
                        DiagnosticSource(diag.path().clone()),
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_content: AlignContent::End,
                            ..default()
                        },
                    ))
                    .with_child((
                        TextSpan::new(diag.path().to_string()),
                        text_font.clone(),
                        text_color.clone(),
                    ))
                    .with_child((TextSpan::new(" N/A"), text_font.clone(), text_color.clone()))
                    .with_child((
                        TextSpan::new(format!(" {}", diag.suffix)),
                        text_font,
                        text_color,
                    ));
            }
        });
}

fn update_ui(diagnostics: Res<DiagnosticsStore>, mut q: Query<(&DiagnosticSource, &mut TextSpan)>) {
    for (diag, mut span) in &mut q {
        if let Some(value) = diagnostics
            .get(&diag.0)
            .and_then(|d| d.is_enabled.then(|| d.value()).flatten())
        {
            **span = format!(" {value}");
        } else {
            **span = " (deactivated)".to_string();
        }
    }
}
