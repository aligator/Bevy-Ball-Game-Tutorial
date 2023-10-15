use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

pub fn interact_with_play_button(
    mut button_query: Query<(&Interaction), (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            app_state_next_state.set(AppState::Game);
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<(&Interaction), (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            app_exit_event_writer.send(AppExit);
        }
    }
}


pub fn animate_buttons(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}