// Imports.
use event;
use GUI;
use Context;
use GameResult;
use MouseButton;

///
/// Struct used to represent the Game Menu.
///
/// - autoplay:     to indicate if autoplay was pressed
/// - draw_once:    used to draw the Game Menu just once
/// - sound:        to indicate if the sound should be played or not
/// - was_clicked:  to differ between "autoplay" / "play" clicks and the quit_event.
/// - gui:          the information about the gui.
pub struct StartState {
    pub autoplay: bool,
    pub draw_once: bool,
    pub sound: bool,
    pub was_clicked: bool,
    pub gui: GUI
}

impl StartState {
    pub fn new() -> StartState {
        StartState{
            autoplay: false,
            draw_once: false,
            sound: true,
            was_clicked: false,
            gui: GUI::new()
        }
    }
}

impl event::EventHandler for StartState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        /* do nothing */
        Ok(())
    }


    ///
    /// This function draws the Start Screen, but it draws it only once.
    ///
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Set draw_once to {true}.
        if !self.draw_once {
            self.gui.draw_start_screen(ctx, self.sound)?;
            self.draw_once = true;
        }

        Ok(())
    }

    ///
    /// This function handles mouse_events.
    /// Used to select "Play" for human-players and "Autoplay" for the KI run.
    ///
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {

        // If the MouseButton is not Left then return.
        if button != MouseButton::Left {
            return;
        }

        // Button positions. ( Could be moved to the GUI )
        let columns = self.gui.columns;
        let size = self.gui.block_size;
        let width = (columns + 2.0) * size;
        let height = 5.0 * size;

        let play_x = width / 4.0;
        let play_y = 10.0 * size;

        let sound_x = (columns + 2.0) * size * 0.5 - 0.75 * size;
        let sound_y = 16.0 * size;

        let sound_width_height = 1.5 * size;

        let autoplay_x = width / 4.0;
        let autoplay_y = 13.0 * size;

        let x = x as f32;
        let y = y as f32;


        // If the play button was clicked, set autoplay to false.
        if x >= play_x && x <= play_x + width * 0.5 && y >= play_y && y <= play_y + height * 0.5 {
            self.autoplay = false;
            self.was_clicked = true;

            // If the autoplay button was clicked, set autoplay to true.
        } else if x >= autoplay_x && x <= autoplay_x + width * 0.5 && y >= autoplay_y && y <= autoplay_y + height * 0.5 {
            self.autoplay = true;
            self.was_clicked = true;
            // If the sound button was clicked, set it false if it was true and vice versa.
        } else if x >= sound_x && x <= sound_x + sound_width_height && y >= sound_y && y <= sound_y + sound_width_height {
            if self.sound {
                self.sound = false;
                self.gui.draw_start_screen(ctx, self.sound).unwrap();
            } else {
                self.sound = true;
                self.gui.draw_start_screen(ctx, self.sound).unwrap();
            }
            return;
        } else {
            // Return if neither of the buttons was pressed.
            return;
        }

        // Some Key Event was executed so quit the context and goon with the game.
        match ctx.quit() {
            Err(_e) => {
                println!("/gui/start_state.rs, Quit Error occurred.");
            }
            _ => {}
        }
    }
}