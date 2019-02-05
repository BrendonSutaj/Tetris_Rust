///
/// This module represents the GUI of the Tetris game, split into 3 states:
/// Start_state:        representing the game menu
/// Main_state:         representing the game itself
/// Game_over_state:    representing the game_over menu
///
pub mod start_state;
pub mod main_state;
pub mod game_over_state;

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
    pub rows_text_pos: Point2<f32>,
    pub points_text_pos: Point2<f32>,
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
        let rows_text_pos = Point2::new(((columns + 2.0) * block_size) * 0.6, block_size * 16.5);
        let points_text_pos = Point2::new(((columns + 2.0) * block_size) * 0.6, block_size * 19.5);
        let next_piece_pos = Point2::new(columns - 3.0, rows + 3.0);
        let standings_rows_pos = Point2::new(block_size, (rows + 4.0) * block_size);
        let standings_points_pos = Point2::new(block_size, (rows + 6.0) * block_size);

        GUI {
            resources_path,
            block_size,
            columns,
            rows,
            rows_text_pos,
            points_text_pos,
            next_piece_pos,
            standings_rows_pos,
            standings_points_pos
        }
    }


    ///
    /// This function is used to create the Context.
    ///
    /// The Context is the Window / the Screen where everything can be drawn onto.
    ///
    pub fn create_context(&self) -> Context {

        ContextBuilder::new("Tetris", "Brendon.Sutaj")
            .add_resource_path(Path::new(&self.resources_path))
            .window_setup(WindowSetup::default().title("Tetris").icon("/icon.png"))
            .window_mode(WindowMode::default().dimensions((self.columns + 2.0) as u32 * self.block_size as u32, (self.rows + 10.0) as u32 * self.block_size as u32))
            .build().expect("Error in /gui/mod.rs, building the context failed.")
    }


    ///
    /// This function is used to draw the game contents onto the context.
    ///
    /// It does this in a few steps:
    ///     (1) Clear the Screen by graphics::clear(ctx);
    ///     (2) Now draw the board layout and the black background.
    ///     (3) Draw the standings (completed rows and achieved points).
    ///     (4) Draw the actual board and all the pieces on it.
    ///     (5) Draw the next_piece.
    ///     (6) Present everything by graphics::present(ctx); and ggez::timer::yield_now();
    ///
    pub fn draw_content(&self, game: &mut Game, ctx: &mut Context) -> GameResult<()> {
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
    fn draw_layout(&self, ctx: &mut Context) -> GameResult<()> {
        let rows = self.rows;
        let columns = self.columns;
        let size = self.block_size;

        // Drawing a black background calling the rectangle function.
        let background = Rect::new(0.0, 0.0, size * (columns + 2.0), size * (rows + 10.0));
        self.rectangle(ctx, background, BLACK)?;

        // Drawing the grey board background.
        let board_background = Rect::new(size, size, size * columns, size * rows);
        self.rectangle(ctx, board_background, Color::from_rgb(192,192,192))?;

        // And now every second column in dark-grey.
        for col in 1..(columns as i32 + 1) {
            if col % 2 == 0 {
                let dark_grey_columns = Rect::new((col as f32) * size, size, size, size * rows);
                self.rectangle(ctx, dark_grey_columns, Color::from_rgb(128,128,128))?;
            }
        }

        Ok(())
    }


    ///
    /// This function is used to draw the board and all the titles onto the screen.
    ///
    fn draw_board(&self, ctx: &mut Context, board: &Board) -> GameResult<()> {
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
    /// This function draws the the standings completed_rows / achieved_points onto the screen.
    ///
    fn draw_standings(&self, ctx: &mut Context, game: &Game) -> GameResult<()> {
        let size = self.block_size;
        let rows = self.rows;

        // Create the rows and points text.
        // Draw it onto the screen in the same color as the next_piece.
        let color = self.get_color_for_piece_type(&game.next_piece.piece_type);
        let row_text = format!("Rows: {}", game.rows);
        let points_text = format!("Points: {}", game.points);

        // Draw the Text to the screen.
        self.draw_text(ctx, &row_text, color, Point2::new(size, (rows + 4.0) * size), size * 0.6)?;
        self.draw_text(ctx, &points_text, color, Point2::new(size, (rows + 6.0) * size), size * 0.6)?;

        Ok(())
    }


    ///
    /// This function draws the next piece onto the screen.
    ///
    fn draw_next_piece(&self, ctx: &mut Context, piece: &Piece) -> GameResult<()> {
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

        // Draw a circle where the rotation point of the piece is located.
        self.draw_circle(ctx, y + piece.point.x_coordinate as f32, x + piece.point.y_coordinate as f32)?;


        Ok(())
    }


    ///
    /// This function draws a circle at the given coordinates (x,y).
    ///
    fn draw_circle(&self, ctx: &mut Context, x: f32, y: f32) -> GameResult<()> {
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
    fn rectangle(&self, ctx: &mut Context, rect: Rect, color: Color) -> GameResult<()> {

        graphics::set_color(ctx, color)?;
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;

        Ok(())
    }


    ///
    /// This function draws a board_tile onto the screen.
    ///
    fn draw_board_tile(&self, ctx: &mut Context, x: f32, y: f32, piece_type: &PieceType) -> GameResult<()> {
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
    fn draw_text(&self, ctx: &mut Context, text: &str, color: Color, position: Point2<f32>, size: f32) -> GameResult<()> {
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
    fn get_color_for_piece_type(&self, piece_type: &PieceType) -> Color {
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
    pub fn draw_game_over(&self, ctx: &mut Context, game_rows: usize, game_points: usize) -> GameResult<()> {
        let size = self.block_size;
        let rows = self.rows;
        let columns = self.columns;

        // clear the stuff before
        graphics::clear(ctx);

        // Draw Game Over Screen
        let game_over_image = Image::new(ctx, "/game_over_screen.png")?;

        // Scaling the Image to (width // height) = (size * (columns + 2.0) // size * (rows + 10.0))
        let scale_x = size * (columns + 2.0) / game_over_image.width() as f32;
        let scale_y = size * (rows + 10.0) / game_over_image.height() as f32;

        let draw_param = DrawParam{
            src: Rect::one(),
            dest: Point2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Point2::new(scale_x, scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        // Draw the image onto the screen.
        graphics::draw_ex(ctx, &game_over_image, draw_param)?;

        // Draw Rows and Points Text in an orange like color.
        let rows_text = format!("{}", game_rows);
        let points_text = format!("{}", game_points);
        let color = graphics::Color::from_rgb(255, 153, 51);

        let rows_text_pos = self.rows_text_pos.clone();
        let points_text_pos = self.points_text_pos.clone();

        // Draw the text onto the screen.
        self.draw_text(ctx, &rows_text, color, rows_text_pos, size * 1.0)?;
        self.draw_text(ctx, &points_text, color, points_text_pos, size * 1.0)?;


        graphics::present(ctx);

        Ok(())
    }


    ///
    /// This function is used to draw the start screen.
    ///
    pub fn draw_start_screen(&self, ctx: &mut Context, sound: bool) -> GameResult<()> {
        graphics::clear(ctx);

        let size = self.block_size;
        let columns = self.columns;
        let rows = self.rows;
        let image;

        if sound {
            // If the sound should be activated, draw the start_screen with the sound symbol.
            image = Image::new(ctx, "/tetris_with_sound.png")?;
        } else {
            // Otherwise draw it with the no-sound symbol.
            image = Image::new(ctx, "/tetris_without_sound.png")?;
        }

        graphics::set_color(ctx, WHITE)?;
        // The Image was scaled to (width // height) = (size * (columns + 2.0) // size * (rows + 10.0))
        let scale_x = size * (columns + 2.0) / image.width() as f32;
        let scale_y = size * (rows + 10.0) / image.height() as f32;

        let draw_param = DrawParam{
            src: Rect::one(),
            dest: Point2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Point2::new(scale_x, scale_y),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None
        };

        // Draw the image onto the screen.
        graphics::draw_ex(ctx, &image, draw_param)?;
        graphics::present(ctx);

        Ok(())
    }
}