use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::main_menu::components::*;
use crate::main_menu::styles::{BUTTON_STYLE, get_button_text_style, get_title_text_style, IMAGE_STYLE, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR, TITLE_STYLE};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands
        .spawn(
            (
                NodeBundle {
                    style: MAIN_MENU_STYLE,
                    ..default()
                },
                MainMenu {}
            )
        )

        .with_children(|parent| {
            // Title
            parent.spawn(
                NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                }
            ).with_children(|parent| {
                // Image 1
                parent.spawn(
                    ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    }
                );

                // Title Text
                parent.spawn(TextBundle {
                    text: Text {
                        linebreak_behavior: BreakLineOn::NoWrap,
                        sections: vec![
                            TextSection::new(
                                "Bevy Ball Game",
                                get_title_text_style(&asset_server),
                            ),
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });

                // Image 2
                parent.spawn(
                    ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    }
                );
            });

            // Play
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {}
                )
            ).with_children(|parent| {
                // Play Text
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Play".to_string(),
                                style: get_button_text_style(&asset_server),
                            }
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });

            // Quit
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {}
                )
            ).with_children(|parent| {
                // Quit Text
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Quit".to_string(),
                                style: get_button_text_style(&asset_server),
                            }
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        })
        .id();

    main_menu_entity
}