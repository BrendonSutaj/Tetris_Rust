pub mod start_state;
pub mod main_state;

// Imports
use std::env;
use std::path::Path;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{self, WHITE, BLACK, Rect, Color, DrawParam, DrawMode, Font, Text, Image};

use game::Game;
use board::Board;
use pieces::piece::Piece;
use pieces::piece_type::PieceType;

use nalgebra::Point2;



pub struct GUI {
    pub resources_path: String,
    pub block_size: f32,
    pub columns: f32,
    pub rows: f32,
    pub game_over_image_pos: Point2<f32>,
    pub game_over_rows_pos: Point2<f32>,
    pub game_over_points_pos: Point2<f32>,
    pub next_piece_pos: Point2<f32>,
    pub standings_rows_pos: Point2<f32>,
    pub standings_points_pos: Point2<f32>,
}

impl GUI {
    // DEFAULT PARAMETERS FOR THE GUI
    // EXCHANGEABLE
    pub fn new() -> GUI {
        let resources_path = format!("{}/resources", env::current_dir().unwrap().to_str().unwrap());
        let block_size: f32 = 25.0;
        let columns: f32 = 11.0;
        let rows: f32 = 20.0;
        let game_over_image_pos = Point2::new(0.0, 0.0);
        let game_over_rows_pos = Point2::new(block_size, block_size * 13.0);
        let game_over_points_pos = Point2::new(block_size, block_size * 16.0);
        let next_piece_pos = Point2::new(columns - 3.0, rows + 3.0);
        let standings_rows_pos = Point2::new(block_size, (rows + 4.0) * block_size);
        let standings_points_pos = Point2::new(block_size, (rows + 6.0) * block_size);

        GUI {
            resources_path,
            block_size,
            columns,
            rows,
            game_over_image_pos,
            game_over_rows_pos,
            game_over_points_pos,
            next_piece_pos,
            standings_rows_pos,
            standings_points_pos
        }
    }


    ///
    /// This function is used to create the Context where everything will be drawn.
    ///
    pub fn create_context(&self) -> Context {

        ContextBuilder::new("Tetris", "Brendon.Sutaj")
            .add_resource_path(Path::new(&self.resources_path))
            .window_setup(WindowSetup::default().title("Tetris").icon("/icon.png"))
            .window_mode(WindowMode::default().dimensions((self.columns + 2.0) as u32 * self.block_size as u32, (self.rows + 10.0) as u32 * self.block_size as u32))
            .build().expect("Building context failed, Error in /gui/mod.rs.")
    }


    ///
    /// This function is used to draw everything at once onto the screen.
    ///
    pub fn draw_content(&mut self, game: &mut Game, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.draw_layout(ctx)?;
        self.draw_standings(ctx, &game)?;
        self.draw_board(ctx, &game.board)?;
        self.draw_next_piece(ctx, &game.next_piece)?;

        graphics::present(ctx);
        ggez::timer::yield_now();

        Ok(())
    }


    ///
    /// This function draws the layout of the board onto the screen.
    ///
    fn draw_layout(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rows = self.rows;
        let columns = self.columns;
        let size = self.block_size;

        // Black Background
        let background = Rect::new(0.0, 0.0, size * (columns + 2.0), size * (rows + 10.0));
        self.rectangle(ctx, background, BLACK)?;

        // Grey Board-Background
        let board_background = Rect::new(size, size, size * columns, size * rows);
        self.rectangle(ctx, board_background, Color::from_rgb(192,192,192))?;

        // Every second Column in Dark Grey
        for col in 1..(columns as i32 + 1) {
            if col % 2 == 0 {
                let dark_grey_columns = Rect::new((col as f32) * size, size, size, size * rows);
                self.rectangle(ctx, dark_grey_columns, Color::from_rgb(128,128,128))?;
            }
        }

        Ok(())
    }


    ///
    /// This function draws the board to the screen.
    ///
    fn draw_board(&mut self, ctx: &mut Context, board: &Board) -> GameResult<()> {
        for x in 0..board.rows {
            for y in 0..board.columns {
                // Draw every tile of the board.
                if board.board[x][y] != PieceType::None {
                    self.draw_board_tile(ctx, x as f32, y as f32, &board.board[x][y])?;
                }
            }
        }

        Ok(())
    }


    ///
    /// This function draws the the standings(rows, points) while the game is running.
    ///
    fn draw_standings(&mut self, ctx: &mut Context, game: &Game) -> GameResult<()> {
        let size = self.block_size;
        let rows = self.rows;

        // The Rows and Points Text to draw.
        let color = self.get_color_for_piece_type(&game.next_piece.piece_type);
        let row_text = format!("Rows: {}", game.rows);
        let points_text = format!("Points: {}", game.points);

        // Draw the Text to the screen.
        self.draw_text(ctx, &row_text, color, Point2::new(size, (rows + 4.0) * size), size * 0.6)?;
        self.draw_text(ctx, &points_text, color, Point2::new(size, (rows + 6.0) * size), size * 0.6)?;

        Ok(())
    }


    ///
    /// This function is used to draw the sound symbol.
    ///
    pub fn draw_sound(&mut self, ctx: &mut Context, sound: bool) -> GameResult<()> {

        let size = self.block_size;
        let columns = self.columns;

        // Load the right image
        let mut image= Image::new(ctx, "/sound.png")?;
        if !sound {
            image = Image::new(ctx, "/no_sound.png")?;
        }

        // Scale the image.
        let scale_x =  (1.5 * size)/image.width() as f32;
        let scale_y = (1.5 * size)/image.height() as f32;
        let dest_x = (columns + 2.0) * size * 0.5 - 0.75 * size;

        let draw_params = DrawParam{
            src: Rect::one(),
            dest: Point2::new(dest_x, 16.0 * size),
            rotation: 0.0,
            scale: Point2::new(scale_x, scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None,
        };

        graphics::set_color(ctx, WHITE)?;
        graphics::draw_ex(ctx, &image, draw_params)?;

        Ok(())

    }

    ///
    /// This function draws the next piece onto the screen.
    ///
    fn draw_next_piece(&mut self, ctx: &mut Context, piece: &Piece) -> GameResult<()> {
        // If the piece_type is NONE don't draw anything.
        if piece.piece_type == PieceType::None {
            return Ok(());
        }

        let x = self.next_piece_pos.coords[0];
        let y = self.next_piece_pos.coords[1];
        let size = self.block_size;

        // Draw every single piece_tile.
        for i in 0..piece.body.rows {
            for j in 0..piece.body.columns {
                if piece.body.data[i][j] {
                    self.draw_board_tile(ctx, y + i as f32, x + j as f32, &piece.piece_type)?;
                }
            }
        }

        // Draw "Next Piece" Text.
        let position = Point2::new(x * size, (y - 1.0) * size);
        let color = self.get_color_for_piece_type(&piece.piece_type);
        self.draw_text(ctx, "Next Piece:", color, position, size * 0.6)?;

        // Draw a circle on the rotation point of the piece.
        self.draw_circle(ctx, y + piece.point.x_coordinate as f32, x + piece.point.y_coordinate as f32)?;


        Ok(())
    }


    ///
    /// This function draws a circle at the given coordinates (x,y).
    ///
    fn draw_circle(&mut self, ctx: &mut Context, x: f32, y: f32) -> GameResult<()> {
        let size = self.block_size;

        // Set the color to WHITE and draw the circle onto the screen.
        graphics::set_color(ctx, WHITE)?;
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point2::new(1.0 + size/2.0 + (y + 1.0) *size, 1.0 + size/2.0 + (x + 1.0) *size),
            (size - 1.0)/4.0,
            2.0
        )?;

        Ok(())
    }


    ///
    /// This function draws a rectangle onto the screen with the given color.
    ///
    pub fn rectangle(&mut self, ctx: &mut Context, rect: Rect, color: Color) -> GameResult<()> {

        graphics::set_color(ctx, color)?;
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;

        Ok(())
    }


    ///
    /// This function draws a board_tile onto the screen.
    ///
    pub fn draw_board_tile(&mut self, ctx: &mut Context, x: f32, y: f32, piece_type: &PieceType) -> GameResult<()> {
        let size = self.block_size;

        // Get the right color for the given piece_type.
        let color = self.get_color_for_piece_type(piece_type);

        // Set the right tile_coordinates and draw the rectangle onto the screen.
        let tile_coordinates = Rect::new(1.0 + (y + 1.0) * size,1.0 + (x + 1.0) * size,size - 1.0,size - 1.0);
        self.rectangle(ctx, tile_coordinates, color)?;

        Ok(())
    }


    ///
    /// This function draws the text onto the screen.
    ///
    pub fn draw_text(&mut self, ctx: &mut Context, text: &str, color: Color, position: Point2<f32>, size: f32) -> GameResult<()> {
        // Load a special font for the text.
        let font = Font::new(ctx, "/DejaVuSerif.ttf", size as u32)?;
        let text_to_draw = Text::new(ctx,text, &font)?;

        // Set the color and draw the text to the screen.
        graphics::set_color(ctx, color)?;
        graphics::draw(ctx, &text_to_draw, position.clone(), 0.0)?;

        Ok(())
    }


    ///
    /// This function returns a color for the given piece_type.
    ///
    fn get_color_for_piece_type(&mut self, piece_type: &PieceType) -> Color {
        match piece_type {
            PieceType::I => Color::from_rgb(0, 255, 255),           // lightblue
            PieceType::J => Color::from_rgb(0, 0, 255),             // blue
            PieceType::L => Color::from_rgb(255,165, 0),            // orange
            PieceType::O => Color::from_rgb(255, 255, 0),           // yellow
            PieceType::S => Color::from_rgb(128, 255, 0),           // green
            PieceType::T => Color::from_rgb(128, 0, 128),           // purple
            PieceType::Z => Color::from_rgb(255, 0, 0),             // red
            PieceType::None => WHITE,                                         // white
        }
    }


    ///
    /// This function draws the game_over_screen with the achieved points and the completed rows.
    ///
    pub fn draw_game_over(&mut self, ctx: &mut Context, game: &Game) -> GameResult<()> {
        let size = self.block_size;
        let rows = self.rows;
        let columns = self.columns;

        // clear the stuff before
        graphics::clear(ctx);

        // Draw Black Background
        let background = Rect::new(0.0, 0.0, size * (columns + 2.0), size * (rows + 10.0));
        self.rectangle(ctx, background, BLACK)?;

        // Draw Rows and Points Text
        let rows_text = format!("Completed Rows: {}", game.rows);
        let points_text = format!("Achieved Points: {}", game.points);
        let color = graphics::Color::from_rgb(255, 69, 0);

        let game_over_rows_position = self.game_over_rows_pos.clone();
        let game_over_points_position = self.game_over_points_pos.clone();

        self.draw_text(ctx, &rows_text, color, game_over_rows_position, size * 0.9)?;
        self.draw_text(ctx, &points_text, color, game_over_points_position, size * 0.9)?;

        // Draw the Game Over Image
        let image = Image::new(ctx, "/game_over_image.png")?;

        // Scale the image.
        let scale_x =  ((columns + 2.0) * size)/image.width() as f32;
        let scale_y = (6.0 * size)/image.height() as f32;

        let draw_params = DrawParam{
            src: Rect::one(),
            dest: Point2::new(0.0, 3.0 * size),
            rotation: 0.0,
            scale: Point2::new(scale_x, scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &image, draw_params)?;

        // Draws everything to the screen.
        graphics::present(ctx);

        // Sleep a while to display the screen long enough.
        ggez::timer::sleep(ggez::timer::f64_to_duration(7.0));

        Ok(())
    }


    pub fn draw_start_screen(&mut self, ctx: &mut Context, sound: bool) -> GameResult<()> {
        let size = self.block_size;
        let rows = self.rows;
        let columns = self.columns;

        // Clear everything from the screen.
        graphics::clear(ctx);



        // Draw Black Background
        let background = Rect::new(0.0, 0.0, size * (columns + 2.0), size * (rows + 10.0));
        self.rectangle(ctx, background, BLACK)?;


        // Draw the tetris image onto the screen.
        let tetris_image = Image::new(ctx, "/tetris.png")?;
        graphics::set_color(ctx, WHITE)?;

        // Scale the image.
        let scale_x =  (columns + 2.0) * size;
        let scale_y = 5.0 * size;

        let draw_params = DrawParam{
            src: Rect::one(),
            dest: Point2::new(0.0, 3.0 * size),
            rotation: 0.0,
            scale: Point2::new(scale_x/tetris_image.width() as f32, scale_y/tetris_image.height() as f32),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &tetris_image, draw_params)?;


        // Draw the start image to the screen.
        let start_image = Image::new(ctx, "/start.png")?;
        graphics::set_color(ctx, WHITE)?;
        let start_image_scale_x = scale_x * 0.5 / start_image.width() as f32;
        let start_image_scale_y = scale_y * 0.5 / start_image.height() as f32;
        let start_image_dest_x = scale_x * 0.25;
        let start_image_dest_y = 7.0 * size;

        let draw_params = DrawParam {
            src: Rect::one(),
            dest: Point2::new(start_image_dest_x, start_image_dest_y + 3.0 * size),
            rotation: 0.0,
            scale: Point2::new(start_image_scale_x, start_image_scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &start_image, draw_params)?;


        // Draw the computer image to the screen.
        let computer_image = Image::new(ctx, "/computer.png")?;
        graphics::set_color(ctx, WHITE)?;

        let computer_image_scale_x = scale_x * 0.5 / computer_image.width() as f32;
        let computer_image_scale_y = scale_y * 0.5 / computer_image.height() as f32;
        let computer_image_dest_x = scale_x * 0.25;
        let computer_image_dest_y = start_image_dest_y + scale_y * 0.5 + 0.5 * size;

        let draw_params = DrawParam {
            src: Rect::one(),
            dest: Point2::new(computer_image_dest_x, computer_image_dest_y + 3.0 * size),
            rotation: 0.0,
            scale: Point2::new(computer_image_scale_x, computer_image_scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &computer_image, draw_params)?;



        // Draw the controls image to the screen.
        let controls_image = Image::new(ctx, "/controls.png")?;
        graphics::set_color(ctx, WHITE)?;

        let controls_image_scale_x = scale_x * 0.5 / controls_image.width() as f32;
        let controls_image_scale_y = scale_y * 0.5 / controls_image.height() as f32;
        let controls_image_dest_x = scale_x * 0.25;
        let controls_image_dest_y = computer_image_dest_y + scale_y * 0.5 + 6.0 * size;

        let draw_params = DrawParam {
            src: Rect::one(),
            dest: Point2::new(controls_image_dest_x, controls_image_dest_y),
            rotation: 0.0,
            scale: Point2::new(controls_image_scale_x, controls_image_scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &controls_image, draw_params)?;


        // Draw the arrow_keys image to the screen.
        let arrow_keys_image = Image::new(ctx, "/arrow_keys.png")?;
        graphics::set_color(ctx, WHITE)?;

        let arrow_keys_image_scale_x = scale_x * 0.5 / arrow_keys_image.width() as f32;
        let arrow_keys_image_scale_y = scale_x * 0.5 * 0.67 / arrow_keys_image.height() as f32;
        let arrow_keys_image_dest_x = 0.0;
        let arrow_keys_image_dest_y = controls_image_dest_y + scale_y * 0.5 + 3.0 * size;

        let draw_params = DrawParam {
            src: Rect::one(),
            dest: Point2::new(arrow_keys_image_dest_x, arrow_keys_image_dest_y),
            rotation: 0.0,
            scale: Point2::new(arrow_keys_image_scale_x, arrow_keys_image_scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &arrow_keys_image, draw_params)?;





        // Draw the xy_keys image to the screen.
        let xy_keys_image = Image::new(ctx, "/xy_keys.png")?;
        graphics::set_color(ctx, WHITE)?;

        let xy_keys_image_scale_x = scale_x * 0.5 / xy_keys_image.width() as f32;
        let xy_keys_image_scale_y = scale_x * 0.5 * 0.67 / xy_keys_image.height() as f32;
        let xy_keys_image_dest_x = scale_x * 0.55;
        let xy_keys_image_dest_y = controls_image_dest_y + scale_y * 0.5 + 3.0 * size;

        let draw_params = DrawParam {
            src: Rect::one(),
            dest: Point2::new(xy_keys_image_dest_x , xy_keys_image_dest_y + 0.5 * size),
            rotation: 0.0,
            scale: Point2::new(xy_keys_image_scale_x * 0.75, xy_keys_image_scale_y * 0.75),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        graphics::draw_ex(ctx, &xy_keys_image, draw_params)?;

        // Draw the sound / no_sound symbol.
        self.draw_sound(ctx, sound)?;

        // Draws everything to the screen.
        graphics::present(ctx);
        Ok(())
    }
}
