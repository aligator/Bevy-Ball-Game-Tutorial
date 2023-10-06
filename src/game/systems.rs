use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && app_state.get() == &AppState::Game {
        if simulation_state.get() == &SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation paused");
        } else if simulation_state.get() == &SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation running");
        }
    }
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.get() != &AppState::Game {
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        commands.insert_resource(NextState(Some(AppState::Game)));
        println!("Entered AppState::Game");
    }
}

pub fn transition_to_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.get() != &AppState::MainMenu {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
        println!("Entered AppState::MainMenu");
    }
}
