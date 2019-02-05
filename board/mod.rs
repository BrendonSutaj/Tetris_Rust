// Imports
use utility::point::Point;
use pieces::piece::Piece;
use pieces::piece_type::PieceType;


// Make the struct cloneable, printable and comparable.
// Board consists of rows, columns and the matrix implemented as a Vec<Vec<PieceType>>.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub rows: usize,
    pub columns: usize,
    pub board: Vec<Vec<PieceType>>
}


impl Board {
    // Initialize the board with PieceType::None.
    pub fn new(rows: usize, columns: usize) -> Board {
        Board {
            rows,
            columns,
            board: vec![vec![PieceType::None; columns]; rows]
        }
    }


    ///
    /// This function checks if we can add a piece at the given row, column.
    ///
    pub fn can_add_piece(&self, piece: &Piece, row: usize, column: usize) -> bool {
        let columns = piece.body.columns;
        let rows = piece.body.rows;

        // Return false if the given row and column has an incorrect value.
        if row >= self.rows || column >= self.columns || row < piece.point.x_coordinate || column < piece.point.y_coordinate {
            return false;
        }

        // Start_point is the top left corner of the piece on the board.
        // End_point is the bottom right corner of the piece on the board.
        let start_point = Point::new(row - piece.point.x_coordinate, column - piece.point.y_coordinate);
        let end_point = Point::new(start_point.x_coordinate + (rows - 1), start_point.y_coordinate + (columns - 1));

        // Return false if the top left or the bottom right corner isn't on the board.
        if start_point.x_coordinate >= self.rows || start_point.y_coordinate >= self.columns {
            return false;
        }

        if end_point.x_coordinate >= self.rows || end_point.y_coordinate >= self.columns {
            return false;
        }

        // If the board has PieceType::None values on every coordinate the piece has a true value in its body => the piece can be added to the board.
        for x in 0..rows {
            for y in 0..columns {
                if piece.body.data[x][y] {
                    if self.board[x + start_point.x_coordinate][y + start_point.y_coordinate] != PieceType::None {
                        return false;
                    }
                }
            }
        }

        true
    }


    ///
    /// This function adds the given piece at the given row, column.
    ///
    pub fn add_piece(&mut self, piece: &Piece, row: usize, column: usize) -> bool {
        // If the piece can be added at the given coordinates goon, return false otherwise.
        if !self.can_add_piece(piece, row, column) {
            return false;
        }

        // Start_point is the top left corner of the piece on the board.
        let start_point = Point::new(row - piece.point.x_coordinate, column - piece.point.y_coordinate);

        // Add the piece_types to the board.
        for x in 0..piece.body.rows {
            for y in 0..piece.body.columns {
                if piece.body.data[x][y] {
                    // We need to clone here unfortunately.
                    self.board[x + start_point.x_coordinate][y + start_point.y_coordinate] = piece.piece_type.clone();
                }
            }
        }

        true
    }


    ///
    /// This function checks if we can remove the given piece and the given row, column.
    ///
    pub fn can_remove_piece(&self, piece: &Piece, row: usize, column: usize) -> bool {
        let columns = piece.body.columns;
        let rows = piece.body.rows;
        let piece_type = piece.piece_type.clone();

        // If the given row, column values are incorrect, return false.
        if row >= self.rows || column >= self.columns {
            return false;
        }

        // Start_point is the top left corner of the piece on the board.
        // End_point is the bottom right corner of the piece on the board.
        let start_point = Point::new(row - piece.point.x_coordinate, column - piece.point.y_coordinate);
        let end_point = Point::new(start_point.x_coordinate + (rows - 1), start_point.y_coordinate + (columns - 1));

        // If the top left or bottom right corner is not on the board, return false.
        if start_point.x_coordinate >= self.rows || start_point.y_coordinate >= self.columns {
            return false;
        }

        if end_point.x_coordinate >= self.rows || end_point.y_coordinate >= self.columns {
            return false;
        }

        // If every coordinate on the board where we want to remove the piece from has the piece_type of the piece given, return true and otherwise false.
        for x in 0..rows {
            for y in 0..columns {
                if piece.body.data[x][y] {
                    if self.board[x + start_point.x_coordinate][y + start_point.y_coordinate] != piece_type {
                        return false;
                    }
                }
            }
        }

        true
    }


    ///
    /// This function removes the given piece at the given row, column.
    ///
    pub fn remove_piece(&mut self, piece: &Piece, row: usize, column: usize) -> bool {
        // Check first if the piece is removable.
        if !self.can_remove_piece(piece, row, column) {
            return false;
        }

        // Start_point is the top left corner of the piece on the board.
        let start_point = Point::new(row - piece.point.x_coordinate, column - piece.point.y_coordinate);

        // Set every board tile to PieceType::None where the piece was at to remove the piece off the board.
        for x in 0..piece.body.rows {
            for y in 0..piece.body.columns {
                if piece.body.data[x][y] {
                    self.board[x + start_point.x_coordinate][y + start_point.y_coordinate] =  PieceType::None;
                }
            }
        }

        true
    }


    ///
    /// This function is used to delete completed rows and return the row count it deleted in this function call.
    ///
    pub fn delete_complete_rows(&mut self) -> usize {
        let columns = self.columns;
        let rows = self.rows;
        let mut complete_rows = Vec::new();
        let mut flag;

        // This procedure detects the completed rows and stores the row in the complete_rows vector.
        for x in 0..rows {
            flag = true;
            for y in 0..columns {
                if self.board[x][y] == PieceType::None {
                    flag = false;
                    break;
                }
            }
            if flag {
                complete_rows.push(x);
            }
        }

        // Create a new vector where we store the new board in.
        let mut new_board = Vec::new();
        let mut row = rows - 1;
        let mut column = 0;

        // Go through the actual board and ignore all rows where the row is contained in the complete_rows vector.
        for x in (0..rows).rev() {
            if complete_rows.contains(&x) {
                continue;
            }
            for y in (0..columns).rev() {
                new_board.push(self.board[x][y].clone());
                column = column + 1;
            }
            row = (row + rows - 1) % rows;
        }

        for _j in 0..complete_rows.len()*columns {
            new_board.push(PieceType::None);
        }

        // Reverse the board, because we had to run it from the bottom to the top.
        new_board.reverse();

        // Now change our actual board, to the board with the completed rows deleted.
        for x in 0..rows {
            for y in 0..columns {
                self.board[x][y] = new_board[x*columns + y].clone();
            }
        }

        // The length of the complete_rows vector is the row_count that was deleted.
        complete_rows.len()
    }
}
