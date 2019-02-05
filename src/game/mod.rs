pub mod move_direction;


// Imports
use board::Board;
use utility::point::Point;
use pieces::piece::Piece;
use pieces::piece_type::PieceType;
use self::move_direction::MoveDirection;
use pieces;


// Make the game cloneable, printable and comparable.
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub rows: usize,
    pub points: usize,
    pub spawn_position: Point,
    pub actual_piece: Piece,
    pub next_piece: Piece,
    pub piece_landed: bool,
    pub actual_piece_coordinates: Point,
}


impl Game {
    pub fn new(board: Board) -> Game {
        let columns = board.columns;
        Game {
            board,
            rows: 0,
            points: 0,
            spawn_position: Point::new(2, columns / 2),
            actual_piece: Piece::empty_piece(),
            next_piece: Piece::empty_piece(),
            piece_landed: false,
            actual_piece_coordinates: Point::empty_point(),
        }
    }


    ///
    /// This function is called whenever a piece lands.
    ///
    /// It deletes completed rows, updates the completed rows counter "rows" and the points achieved so far "points".
    /// After that, it adds the "actual_piece" at the spawn position (2, board.columns / 2) onto the board.
    ///
    /// Returns {true} if the "actual_piece" could be added to the board, {false} otherwise.
    ///
    pub fn new_piece(&mut self) -> bool {
        // Get the amount of deleted rows.
        let amount_of_deleted_rows = self.board.delete_complete_rows();

        // Update the rows completed so far.
        self.rows += amount_of_deleted_rows;

        //Update the points accordingly.
        match amount_of_deleted_rows {
            0 => {/* Do nothing here. */},
            1 => self.points += 100,
            2 => self.points += 300,
            3 => self.points += 500,
            4 => self.points += 1000,
            _ => panic!("Error in /game/mod.rs, impossible amount of rows deleted.")
        }

        // Add the new piece onto the board and reset "piece_landed" to {false}.
        if self.board.add_piece(&self.actual_piece, self.spawn_position.x_coordinate, self.spawn_position.y_coordinate) {
            self.actual_piece_coordinates = self.spawn_position.clone();
            self.piece_landed = false;

            // The "actual_piece" could be added, return {true}.
            return true;
        }


        // The "actual_piece" couldn't be added, return {false}.
        false
    }


    ///
    /// This function checks if the game is in an game over state.
    ///
    /// If the "actual_piece" can't be added and the previous piece landed ("piece_landed") return {true}
    /// return {false} otherwise.
    ///
    pub fn is_game_over(&self) -> bool {
        !self.board.can_add_piece(&self.actual_piece, self.spawn_position.x_coordinate, self.spawn_position.y_coordinate) && self.piece_landed
    }


    ///
    /// This function is used to rotate the actual piece clockwise.
    ///
    /// First the "actual_piece" has to be removed. After that a clone of the "actual_piece" has to be rotated.
    ///
    /// If the rotated piece can be added to the board, return {true} and set the "actual_piece" to the rotated one,
    /// return {false} otherwise and add the "actual_piece" again to the board.
    ///
    pub fn rotate_piece_clockwise(&mut self) -> bool {
        // Remove the "actual_piece" off the board.
        if self.board.remove_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
            // Create a clone of the "actual_piece", because we don't want to change it.
            let mut rotated_piece = self.actual_piece.clone();
            // Now compute the rotation.
            rotated_piece.perform_clockwise_rotation();

            // Add the rotated piece to the board, if it can be added.
            if self.board.add_piece(&rotated_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
                // Set the "actual_piece" to the rotated one and return {true}.
                self.actual_piece = rotated_piece.clone();
                return true;
            } else {
                // If we cant add the rotated piece, add the "actual_piece" again and return {false}.
                self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
            }
        }

        false
    }


    ///
    /// This function is used to rotate the actual piece counter-clockwise.
    ///
    /// First the "actual_piece" has to be removed. After that a clone of the "actual_piece" has to be rotated.
    ///
    /// If the rotated piece can be added to the board, return {true} and set the "actual_piece" to the rotated one,
    /// return {false} otherwise and add the "actual_piece" again to the board.
    ///
    pub fn rotate_piece_counter_clockwise(&mut self) -> bool {
        // Remove the "actual_piece" off the board.
        if self.board.remove_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
            // Create a clone of the "actual_piece", because we don't want to change it.
            let mut rotated_piece = self.actual_piece.clone();
            // Now compute the rotation.
            rotated_piece.perform_counter_clockwise_rotation();

            // Add the rotated piece to the board, if it can be added.
            if self.board.add_piece(&rotated_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
                // Set the "actual_piece" to the rotated one and return {true}.
                self.actual_piece = rotated_piece.clone();
                return true;
            } else {
                // If we cant add the rotated piece, add the "actual_piece" again and return {false}.
                self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
            }
        }

        false
    }


    ///
    /// This function is used to move the "actual_piece" to the given "move_direction".
    ///
    /// It returns {true} if the move was successful, return {false} otherwise.
    ///
    pub fn move_in_direction(&mut self, move_direction: MoveDirection) -> bool {
        // "Actual_piece" coordinates.
        let mut x_coordinate = self.actual_piece_coordinates.x_coordinate;
        let mut y_coordinate = self.actual_piece_coordinates.y_coordinate;

        // First remove the "actual_piece" at the coordinates given above.
        if self.board.remove_piece(&self.actual_piece, x_coordinate, y_coordinate) {
            // Match the move_direction and compute the new coordinates accordingly.
            // Add the piece at the new coordinates, return {true} on success.
            // Refuse adding the piece to the board at incorrect coordinates.
            match move_direction {
                MoveDirection::Down => {
                    if x_coordinate >= self.board.rows - 1 {
                        self.board.add_piece(&self.actual_piece, x_coordinate, y_coordinate);
                        return false;
                    }
                    x_coordinate += 1
                },
                MoveDirection::Left => {
                    if y_coordinate <= 0 {
                        self.board.add_piece(&self.actual_piece, x_coordinate, y_coordinate);
                        return false;
                    }
                    y_coordinate -= 1
                }
                MoveDirection::Right => {
                    if y_coordinate >= self.board.columns - 1 {
                        self.board.add_piece(&self.actual_piece, x_coordinate, y_coordinate);
                        return false;
                    }
                    y_coordinate += 1
                }
            }

            // Add the piece at the new coordinates, update the "actual_piece_coordinates" and return {true}.
            if self.board.add_piece(&self.actual_piece, x_coordinate, y_coordinate) {
                self.actual_piece_coordinates = Point::new(x_coordinate, y_coordinate);
                return true;
            } else {
                // Otherwise add the "actual_piece" back to the old location and return {false}.
                self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
            }
        }

        false
    }


    ///
    /// This function is used to perform a logical game step in the given "move_direction".
    ///
    /// It returns {true} if a new piece was added and {false} otherwise.
    ///
    pub fn step(&mut self, move_direction: MoveDirection) -> bool {
        // If the actual piece does not exist, create a new actual and next piece.
        if self.actual_piece.piece_type == PieceType::None {
            self.actual_piece = pieces::get_next_random_piece();
            self.next_piece = pieces::get_next_random_piece();
        }

        // If the "move_direction" is "Down", check if the piece has landed. (Move doesn't work.)
        // If the "actual_piece" has landed, replace the "actual piece" with the "next piece".
        // The "next_piece" gets replaced by a new random next piece.
        // Call new_piece() after that, to update the amount of completed rows and points.
        if move_direction == MoveDirection::Down {
            if !self.move_in_direction(move_direction) {
                self.piece_landed = true;
                self.actual_piece = self.next_piece.clone();
                self.next_piece = pieces::get_next_random_piece();
                self.new_piece();
                return true;
            }
        } else {
            // If the MoveDirection is not down, move in the given direction.
            self.move_in_direction(move_direction);
        }

        false
    }
}