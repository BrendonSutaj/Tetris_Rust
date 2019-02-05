// Crates included. ggez => gui // rand => randomness for piece_spawns.
extern crate ggez;
extern crate rand;

// Imports
use ggez::{Context, GameResult};
use ggez::audio;
use ggez::event::{self, Keycode, Mod, MouseButton};
use ggez::nalgebra;
use ggez::timer;

use autoplayer::Autoplayer;
use autoplayer::commands::Command;
use board::Board;
use game::Game;
use game::move_direction::MoveDirection;
use gui::game_over_state::GameOverState;
use gui::GUI;
use gui::main_state::MainState;
use gui::start_state::StartState;

mod board;
mod pieces;
mod utility;
mod game;
mod gui;
mod autoplayer;

fn main() {

    // Create the gui, the game and the auto_player.
    let mut completed_rows = 0;
    let mut achieved_points = 0;

    // Create the context with the values given in the gui.
    let ctx = &mut GUI::new().create_context();


    // Loop every time restart was pressed in the game over menu.
    loop {

        // Create the start state and run the events_loop.
        // Its used to describe the game controls and to decide if the ki or the human wants to play the game.
        let start_state = &mut StartState::new();
        ggez::event::run(ctx, start_state).unwrap();

        // If a quit event has occurred present the game_over_screen, else keep going.
        if start_state.was_clicked {

            // Start playing a song as soon as the game starts.
            let mut audio = audio::Source::new(ctx, "/tetris_theme_song.ogg").unwrap();
            audio.set_repeat(true);

            if start_state.sound {
                match audio.play() {
                    Err(_e) => {
                        println!("Audio Error, Play() in /main.rs");
                    },
                    _ => {}
                };
            }

            // Create the state and run the events_loop.
            let main_state = &mut MainState::new(start_state.autoplay);
            ggez::event::run(ctx, main_state).unwrap();

            // Pause when the game is over.
            if start_state.sound {
                audio.stop();
            }

            // Update completed_rows and achieved_points.
            completed_rows = main_state.game.rows;
            achieved_points = main_state.game.points;
        }

        // The game is over now, so we draw the game_over_screen.
        let game_over_state = &mut GameOverState::new(completed_rows, achieved_points);
        ggez::event::run(ctx, game_over_state).unwrap();

        // Start over if the restart "button" was pressed.
        if !game_over_state.play_again {
          break;
        }
    }
}
