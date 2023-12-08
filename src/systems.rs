use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::resources::Bongo;
use crate::AppState;

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.get_single_mut().unwrap();

    window.resolution.set(800.0, 600.0);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::Game {
        app_state_next_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
        app_state_next_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score);
        app_state_next_state.set(AppState::GameOver);
        println!("Entered AppState::GameOver");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn tick_bongo(time: Res<Time>, mut bongo: ResMut<Bongo>) {
    bongo.tick(time);
}
