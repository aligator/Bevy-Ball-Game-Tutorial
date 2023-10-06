use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    app_state: Res<State<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && app_state.get() == &AppState::Game {
        if simulation_state.get() == &SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation paused");
        } else if simulation_state.get() == &SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation running");
        }
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.get() != &AppState::Game {
        next_app_state.set(AppState::Game);
        next_simulation_state.set(SimulationState::Paused);
        println!("Entered AppState::Game");
    }
}

pub fn transition_to_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.get() != &AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
    }
}
