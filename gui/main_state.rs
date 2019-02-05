// Imports.
use GUI;
use Game;
use Autoplayer;
use Command;
use MoveDirection;
use GameResult;
use Context;
use Keycode;
use Mod;
use Board;

use event;
use utility;
use timer;


///
/// This struct represents the Main_state, where the game itself will happen.
///
/// - gui:                  the information of the GUI
/// - game:                 the game
/// - time_since_start_old: used to compute the automatic move-down correctly
/// - autoplay:             indicates if the autoplayer is running
/// - is_game_over:         indicates if the game is over
/// - auto_player:          the actual auto_player
///
pub struct MainState {
    pub gui: GUI,
    pub game: Game,
    pub time_since_start_old: f64,
    pub autoplay: bool,
    pub is_game_over: bool,
    pub auto_player: Autoplayer,
}


impl MainState {
    pub fn new(autoplay: bool) -> MainState {
        let gui = GUI::new();
        let game = Game::new(Board::new(gui.rows as usize, gui.columns as usize));
        let auto_player = Autoplayer::new();
        MainState {
            gui,
            game,
            time_since_start_old: 0.0,
            autoplay,
            is_game_over: false,
            auto_player,
        }

    }

    ///
    /// This function translates the autoplayer move command to a game step and performs the move.
    ///
    pub fn auto_player_move(&mut self) {

        match self.auto_player.perform_move(&mut self.game) {
            Command::Down => {
                if self.game.step(MoveDirection::Down) { self.auto_player.compute_move(&mut self.game); }
            }
            Command::Left => {
                if self.game.step(MoveDirection::Left) { self.auto_player.compute_move(&mut self.game); }
            }
            Command::Right => {
                if self.game.step(MoveDirection::Right) { self.auto_player.compute_move(&mut self.game); }
            }
            Command::RotateClockWise => {
                self.game.rotate_piece_clockwise();
            }
        }
    }
}

///
/// Implementing the EventHandler for the MainState struct. We have to implement "update()" and "draw()".
/// Implementing Events like "key_events" is optional.
///
impl event::EventHandler for MainState {
    ///
    /// Here is where the game logic takes place.
    ///
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let duration = utility::duration_for_level(&self.game);
        let time_since_start_new = timer::duration_to_f64(timer::get_time_since_start(ctx));

        // Autoplayer is active.
        if self.autoplay {
            if time_since_start_new - self.time_since_start_old >= duration {
                if self.game.step(MoveDirection::Down) {
                    self.auto_player.compute_move(&mut self.game);
                    self.gui.draw_content(&mut self.game, ctx)?;
                }
                self.time_since_start_old = time_since_start_new;
            } else {
                self.auto_player_move();
                self.gui.draw_content(&mut self.game, ctx)?;
            }
            // Human player is active.
        } else {
            if time_since_start_new - self.time_since_start_old >= duration {
                self.game.step(MoveDirection::Down);
                self.time_since_start_old = time_since_start_new;
            }
        }

        // Quit by setting ctx.continue to false.
        if self.game.is_game_over() && !self.is_game_over {
            self.is_game_over = true;
            ctx.quit()?;
        }

        Ok(())
    }

    ///
    /// This function is used to draw the content onto the screen.
    ///
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.gui.draw_content(&mut self.game, ctx)?;

        Ok(())
    }

    ///
    /// This function handles key_events.
    /// Controls are:   Left, Right, Down   - to move the piece in the given direction.
    ///                 Y and X             - to rotate the piece clock- and counter- clockwise.
    ///                 Escape              - to quit the game early.
    ///
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _key_mods: Mod, _repeat: bool) {
        if self.game.is_game_over() {
            return;
        }

        // Match on the keycode of the Key that was pressed.
        match keycode {
            Keycode::Left => {
                if !self.autoplay {
                    self.game.step(MoveDirection::Left);
                }
            }
            Keycode::Right => {
                if !self.autoplay {
                    self.game.step(MoveDirection::Right);
                }
            }
            Keycode::Down => {
                if !self.autoplay {
                    self.game.step(MoveDirection::Down);
                }
            }
            Keycode::Y => {
                if !self.autoplay {
                    self.game.rotate_piece_clockwise();
                }
            }
            Keycode::X => {
                if !self.autoplay {
                    self.game.rotate_piece_counter_clockwise();
                }
            }
            Keycode::Escape => {
                match ctx.quit() {
                    Err(_e) => {
                        println!("Quit Error, EscapeKeyEvent in /main.rs");
                    }
                    _ => {}
                }
            }
            _ => { return; }
        }

        // Draw the content after each key_event. The update function felt too slow.
        match self.gui.draw_content(&mut self.game, ctx) {
            Err(_e) => {
                println!("/main_state/main.rs, Draw Error occurred.");
            }
            _ => {}
        }
    }
}