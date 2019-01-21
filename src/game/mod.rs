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
    /// This function is called whenever a piece landed and checks for deleted rows, updates points, adds the actual piece at the spawn position.
    ///
    pub fn new_piece(&mut self) -> bool {
        // Get the count of deleted rows.
        let deleted_rows_count = self.board.delete_complete_rows();

        // Update the rows completed so far.
        self.rows = self.rows + deleted_rows_count;

        // Update the points.
        match deleted_rows_count {
            0 => self.points += 0,
            1 => self.points += 100,
            2 => self.points += 300,
            3 => self.points += 500,
            4 => self.points += 1000,
            _ => panic!("To many rows deleted Error in game/mod.rs")
        }

        // Add the new piece onto the board and set piece_landed to false again.
        if self.board.add_piece(&self.actual_piece, self.spawn_position.x_coordinate, self.spawn_position.y_coordinate) {
            self.actual_piece_coordinates = self.spawn_position.clone();
            self.piece_landed = false;
            return true;
        }

        // If nothing new was added return false.
        false
    }


    ///
    /// This function checks for game over.
    ///
    pub fn is_game_over(&self) -> bool {
        // If the previous piece landed and the actual piece cant be added to the spawn coordinates return true otherwise false.
        if !self.board.can_add_piece(&self.actual_piece, self.spawn_position.x_coordinate, self.spawn_position.y_coordinate) && self.piece_landed {
            true
        } else {
            false
        }
    }


    ///
    /// This function is used to rotate the actual piece clockwise, and returns true if it was successful, false otherwise.
    ///
    pub fn rotate_piece_clockwise(&mut self) -> bool {
        // Remove the piece off the board.
        if self.board.remove_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
            let mut rotated_piece = self.actual_piece.clone();
            // Compute the rotation.
            rotated_piece.perform_clockwise_rotation();

            // Add the rotated piece to the board.
            if self.board.add_piece(&rotated_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
                self.actual_piece = rotated_piece.clone();
                return true;
            } else {
                // If we cant add the rotated piece, add the previous piece again and return false.
                self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
            }
        }

        false
    }


    ///
    /// This function is used to rotate the piece counter_clockwise.
    ///
    pub fn rotate_piece_counter_clockwise(&mut self) -> bool {
        // Remove the piece off the board.
        if self.board.remove_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
            let mut rotated_piece = self.actual_piece.clone();
            // Compute the rotation.
            rotated_piece.perform_counter_clockwise_rotation();

            // Add the rotated piece to the board.
            if self.board.add_piece(&rotated_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate) {
                self.actual_piece = rotated_piece.clone();
                return true;
            } else {
                // If the rotated piece can't be added, add the previous piece again.
                self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
            }
        }

        false
    }


    ///
    /// This function moved the actual piece at the given MoveDirection and return true on success.
    ///
    pub fn move_in_direction(&mut self, move_direction: MoveDirection) -> bool {
        let mut x_coordinate = self.actual_piece_coordinates.x_coordinate;
        let mut y_coordinate = self.actual_piece_coordinates.y_coordinate;

        // First remove the piece at the actual coordinates.
        if self.board.remove_piece(&self.actual_piece, x_coordinate, y_coordinate) {
            // Match the move_direction and compute the new coordinates accordingly.
            // Add the piece at the new coordinates, return true on success.
            // Refuse adding the piece to the board at incorrect coordinates.
            match move_direction {
                MoveDirection::Down => {
                    if x_coordinate >= self.board.rows - 1 {
                        self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
                        return false;
                    }
                    x_coordinate += 1
                },
                MoveDirection::Left => {
                    if y_coordinate <= 0 {
                        self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
                        return false;
                    }
                    y_coordinate -= 1
                }
                MoveDirection::Right => {
                    if y_coordinate >= self.board.columns - 1 {
                        self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
                        return false;
                    }
                    y_coordinate += 1
                }
            }

            // Add the piece at the new coordinates.
            if self.board.add_piece(&self.actual_piece, x_coordinate, y_coordinate) {
                self.actual_piece_coordinates = Point::new(x_coordinate, y_coordinate);
                return true;
            } else {
                self.board.add_piece(&self.actual_piece, self.actual_piece_coordinates.x_coordinate, self.actual_piece_coordinates.y_coordinate);
            }
        }

        false
    }


    ///
    /// This function performs a logical game step and returns true if a new_piece has spawned, false otherwise.
    ///
    pub fn step(&mut self, move_direction: MoveDirection) -> bool {
        // If the actual piece does not exist, create a new actual and next piece.
        if self.actual_piece.piece_type == PieceType::None {
            self.actual_piece = pieces::get_next_random_piece();
            self.next_piece = pieces::get_next_random_piece();
        }

        // If the MoveDirection is down, check if the piece has landed. (Move does not work)
        // If it has landed, replace the actual piece with the next piece, get a new random next piece and call the new_piece() function.
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