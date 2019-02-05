// Imports.
use event;
use GUI;
use Context;
use GameResult;
use MouseButton;

///
/// This struct represents the game_over_menu.
///
/// - draw_once:        draw the game_over_menu just once
/// - play_again:       indicates if the "restart" button was pressed
/// - game_rows:        completed_rows
/// - game_points:      achieved_points
/// - gui:              the information of the gui
///
pub struct GameOverState {
    pub draw_once: bool,
    pub play_again: bool,
    pub game_rows: usize,
    pub game_points: usize,
    pub gui: GUI
}

impl GameOverState {
    pub fn new(game_rows: usize, game_points: usize) -> GameOverState {
        GameOverState{
            draw_once: false,
            play_again: false,
            game_rows,
            game_points,
            gui: GUI::new()
        }
    }
}


impl event::EventHandler for GameOverState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        /* do nothing */
        Ok(())
    }


    ///
    /// This function draws the Game Over Screen, but it draws it only once.
    ///
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Draws Game Over Screen only once.
        if !self.draw_once {
            self.gui.draw_game_over(ctx, self.game_rows, self.game_points)?;
            self.draw_once = true;
        }

        Ok(())
    }


    ///
    /// This function handles mouse_events.
    /// Used to select "Restart", to restart the game from the main menu again.
    ///
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {

        // If the MouseButton is not Left then return.
        if button != MouseButton::Left {
            return;
        }

        // Button positions. ( Could be moved to the gui )
        let columns = self.gui.columns;
        let size = self.gui.block_size;

        let x = x as f32;
        let y = y as f32;

        let width = (columns + 2.0) * size;
        let height = 3.0 * size;
        let pos_x = 0.0;
        let pos_y = 11.0 * size;


        // If the play button was clicked, set restart to true.
        if x >= pos_x && x <= pos_x + width && y >= pos_y && y <= pos_y + height {
            self.play_again = true;
        } else {
            return;
        }

        // Some Key Event was executed so quit the context and goon with the game.
        match ctx.quit() {
            Err(_e) => {
                println!("/game_over_state/main.rs, Quit Error occurred.");
            }
            _ => {}
        }
    }
}