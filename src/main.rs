// Crates included. ggez => gui // rand => randomness for piece_spawns.
extern crate ggez;
extern crate rand;

mod board;
mod pieces;
mod utility;
mod game;
mod gui;
mod autoplayer;

// Imports
use ggez::{Context, GameResult};
use ggez::nalgebra;
use ggez::timer;
use ggez::audio;
use ggez::event::{self, MouseButton, Keycode, Mod};

use game::move_direction::MoveDirection;
use game::Game;
use board::Board;

use autoplayer::commands::Command;
use autoplayer::Autoplayer;

use gui::GUI;

use gui::start_state::StartState;
use gui::main_state::MainState;


fn main() {

    // Create the gui, the game and the auto_player.
    let gui = GUI::new();
    let game = Game::new(Board::new(gui.rows as usize, gui.columns as usize));
    let auto_player = autoplayer::Autoplayer::new();

    // Create the context with the values given in the gui.
    let ctx = &mut gui.create_context();

    // Create the start state and run the events_loop.
    // Its used to describe the game controls and to decide if the ki or the human wants to play the game.
    let start_state = &mut StartState::new();
    ggez::event::run(ctx, start_state).unwrap();

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
    let main_state = &mut MainState::new(gui, game, start_state.autoplay, auto_player);
    ggez::event::run(ctx, main_state).unwrap();

    // Pause when the game is over.
    if start_state.sound {
        audio.pause();
    }

    // The game is over now, so we draw the game_over_screen.
    match main_state.gui.draw_game_over(ctx,  &main_state.game) {
        Err(_e) => {
            println!("Draw Error, GameOverScreen in /main.rs");
        }
        _ => {}
    }
}
