// Imports
use pieces::piece_type::PieceType;
use utility::matrix::Matrix;
use utility::point::Point;


// Makes this struct cloneable and printable.
// Represents a piece by a piece_type, a boolean Matrix and the rotation point.
#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub body: Matrix,
    pub point: Point,
}


impl Piece {
    pub fn new(piece_type: PieceType, body: Matrix, point: Point) -> Piece {
        Piece {
            piece_type,
            body,
            point,
        }
    }

    ///
    /// This function creates an empty_piece, which is basically a piece with PieceType::None, the other values don't matter.
    ///
    pub fn empty_piece() -> Piece {
        Piece::new(PieceType::None, Matrix::new(1, 1), Point::empty_point())
    }


    ///
    /// This function performs the clockwise rotation.
    ///
    pub fn perform_clockwise_rotation(&mut self) {
        let x_coordinate = self.point.x_coordinate.clone();
        let y_coordinate = self.point.y_coordinate.clone();

        // Set the rotation point accordingly.
        self.point.set(y_coordinate, self.body.rows - 1 - x_coordinate);
        // Rotate the body.
        self.body.rotate_clockwise();
    }


    ///
    /// This function performs the counter clockwise rotation.
    ///
    pub fn perform_counter_clockwise_rotation(&mut self) {
        let x_coordinate = self.point.x_coordinate.clone();
        let y_coordinate = self.point.y_coordinate.clone();

        // Set the rotation point accordingly.
        self.point.set(self.body.columns - 1 - y_coordinate, x_coordinate);
        // Rotate the body.
        self.body.rotate_counter_clockwise();
    }
}