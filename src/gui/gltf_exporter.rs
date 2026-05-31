use crate::core::world::EditorWorld;
use crate::file::gltf::export_world_to_gltf;
use bevy::prelude::*;

#[derive(Component)]
pub struct ExportButtonMarker;

pub struct GltfExporterGuiPlugin;

impl Plugin for GltfExporterGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_export_button_ui)
            .add_systems(Update, handle_export_button_interaction);
    }
}

fn spawn_export_button_ui(mut commands: Commands) {
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                width: Val::Px(160.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::Srgba(Srgba {
                red: 0.15,
                green: 0.15,
                blue: 0.15,
                alpha: 1.0,
            })),
            ExportButtonMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Export Level"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn handle_export_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExportButtonMarker>),
    >,
    world_state: Res<EditorWorld>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::Srgba(Srgba {
                    red: 0.4,
                    green: 0.4,
                    blue: 0.4,
                    alpha: 1.0,
                }));
                println!("Triggered Export from GUI Plugin...");

                export_world_to_gltf(&world_state);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::Srgba(Srgba {
                    red: 0.25,
                    green: 0.25,
                    blue: 0.25,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::Srgba(Srgba {
                    red: 0.15,
                    green: 0.15,
                    blue: 0.15,
                    alpha: 1.0,
                }));
            }
        }
    }
}
