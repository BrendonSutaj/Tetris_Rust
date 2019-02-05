pub mod piece;
pub mod piece_type;

// Imports
use rand::Rng;

use self::piece::Piece;
use self::piece_type::PieceType;

use utility::point::Point;
use utility::matrix::Matrix;

///
/// This module should represent a piece_factory.
///


///
/// This function creates the default IPiece.
///
pub fn get_i_piece() -> Piece {
    let point = Point::new(1, 0);
    let mut matrix = Matrix::new(4, 1);
    matrix.data[0][0] = true;
    matrix.data[1][0] = true;
    matrix.data[2][0] = true;
    matrix.data[3][0] = true;

    Piece::new(PieceType::I,matrix, point)
}


///
/// This function creates the default JPiece.
///
pub fn get_j_piece() -> Piece {
    let point = Point::new(1, 1);
    let mut matrix = Matrix::new(3, 2);
    matrix.data[0][0] = false;
    matrix.data[0][1] = true;
    matrix.data[1][0] = false;
    matrix.data[1][1] = true;
    matrix.data[2][0] = true;
    matrix.data[2][1] = true;

    Piece::new(PieceType::J,matrix, point)
}


///
/// This function creates the default LPiece.
///
pub fn get_l_piece() -> Piece {
    let point = Point::new(1, 0);
    let mut matrix = Matrix::new(3, 2);
    matrix.data[0][0] = true;
    matrix.data[0][1] = false;
    matrix.data[1][0] = true;
    matrix.data[1][1] = false;
    matrix.data[2][0] = true;
    matrix.data[2][1] = true;

    Piece::new(PieceType::L,matrix, point)
}


///
/// This function creates the default OPiece.
///
pub fn get_o_piece() -> Piece {
    let point = Point::new(1, 1);
    let mut matrix = Matrix::new(2, 2);
    matrix.data[0][0] = true;
    matrix.data[0][1] = true;
    matrix.data[1][0] = true;
    matrix.data[1][1] = true;

    Piece::new(PieceType::O,matrix, point)
}


///
/// This function creates the default SPiece.
///
pub fn get_s_piece() -> Piece {
    let point = Point::new(1, 1);
    let mut matrix = Matrix::new(2, 3);
    matrix.data[0][0] = false;
    matrix.data[0][1] = true;
    matrix.data[0][2] = true;
    matrix.data[1][0] = true;
    matrix.data[1][1] = true;
    matrix.data[1][2] = false;

    Piece::new(PieceType::S,matrix, point)
}


///
/// This function creates the default TPiece.
///
pub fn get_t_piece() -> Piece {
    let point = Point::new(0, 1);
    let mut matrix = Matrix::new(2, 3);
    matrix.data[0][0] = true;
    matrix.data[0][1] = true;
    matrix.data[0][2] = true;
    matrix.data[1][0] = false;
    matrix.data[1][1] = true;
    matrix.data[1][2] = false;

    Piece::new(PieceType::T,matrix, point)
}


///
/// This function creates the default ZPiece.
///
pub fn get_z_piece() -> Piece {
    let point = Point::new(1, 1);
    let mut matrix = Matrix::new(2, 3);
    matrix.data[0][0] = true;
    matrix.data[0][1] = true;
    matrix.data[0][2] = false;
    matrix.data[1][0] = false;
    matrix.data[1][1] = true;
    matrix.data[1][2] = true;

    Piece::new(PieceType::Z,matrix, point)
}


///
/// This function creates a new random_piece.
///
pub fn get_next_random_piece() -> Piece {
    let random_number = rand::thread_rng().gen_range(0, 7);
    match random_number {
        0 => get_i_piece(),
        1 => get_j_piece(),
        2 => get_l_piece(),
        3 => get_o_piece(),
        4 => get_s_piece(),
        5 => get_t_piece(),
        6 => get_z_piece(),
        _ => panic!("Random-Generator Error in pieces/mod.rs"),
    }
}



///
/// TESTS BEGIN HERE
///


#[cfg(test)]
mod tests {
    use utility::point::Point;
    use utility::matrix::Matrix;
    use pieces::piece::Piece;
    use pieces::piece_type::PieceType;
    use pieces::*;

    #[test]
    fn piece_i() {
        let point = Point::new(1, 0);
        let mut m = Matrix::new(4, 1);
        m.data[0][0] = true;
        m.data[1][0] = true;
        m.data[2][0] = true;
        m.data[3][0] = true;

        assert_eq!(Piece::new(PieceType::I,m, point), get_i_piece());
    }

    #[test]
    fn piece_j() {
        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = false; m.data[1][1] = true;
        m.data[2][0] = true;  m.data[2][1] = true;

        assert_eq!(Piece::new(PieceType::J,m, point), get_j_piece());
    }

    #[test]
    fn piece_l() {
        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = false;
        m.data[2][0] = true;  m.data[2][1] = true;

        assert_eq!(Piece::new(PieceType::L,m, point), get_l_piece());
    }

    #[test]
    fn piece_o() {
        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m, point), get_o_piece());
    }

    #[test]
    fn piece_s() {
        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = true; m.data[0][2] = true;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2] = false;

        assert_eq!(Piece::new(PieceType::S,m, point), get_s_piece());
    }

    #[test]
    fn piece_t() {
        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2] = true;
        m.data[1][0] = false; m.data[1][1] = true; m.data[1][2] = false;

        assert_eq!(Piece::new(PieceType::T,m, point), get_t_piece());
    }

    #[test]
    fn piece_z() {
        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2] = false;
        m.data[1][0] = false; m.data[1][1] = true; m.data[1][2] = true;

        assert_eq!(Piece::new(PieceType::Z,m, point), get_z_piece());
    }


    #[test]
    fn rotate_clockwise_i(){
        let mut piece = get_i_piece();

        //first rotation
        piece.perform_clockwise_rotation();
        let point = Point::new(0, 2);
        let mut m = Matrix::new(1, 4);
        m.data[0][0] = true; m.data[0][1]=true; m.data[0][2]=true; m.data[0][3]=true;

        assert_eq!(Piece::new(PieceType::I,m, point),piece);

        //second rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(2, 0);
        let mut m = Matrix::new(4, 1);
        m.data[0][0] = true;
        m.data[1][0] = true;
        m.data[2][0] = true;
        m.data[3][0] = true;

        assert_eq!(Piece::new(PieceType::I,m, point),piece);


        //third rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(1, 4);
        m.data[0][0] = true; m.data[0][1]=true; m.data[0][2]=true; m.data[0][3]=true;

        assert_eq!(Piece::new(PieceType::I,m, point),piece);

        //back to start rotation
        piece.perform_clockwise_rotation();
        assert_eq!(get_i_piece(),piece);
    }


    #[test]
    fn rotate_clockwise_j(){
        let mut piece = get_j_piece();

        //first rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = false; m.data[0][2]=false;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2]=true;

        assert_eq!(Piece::new(PieceType::J,m,point),piece);

        //second rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = false;
        m.data[2][0] = true;  m.data[2][1] = false;

        assert_eq!(Piece::new(PieceType::J,m,point),piece);

        //third rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2]=true;
        m.data[1][0] = false; m.data[1][1] = false; m.data[1][2]=true;

        assert_eq!(Piece::new(PieceType::J,m,point),piece);

        //back to start rotation
        piece.perform_clockwise_rotation();
        assert_eq!(get_j_piece(),piece);
    }


    #[test]
    fn rotate_clockwise_l(){
        let mut piece = get_l_piece();

        //first rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2]=true;
        m.data[1][0] = true; m.data[1][1] = false; m.data[1][2]=false;

        assert_eq!(Piece::new(PieceType::L,m,point),piece);

        //second rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = false; m.data[1][1] = true;
        m.data[2][0] = false;  m.data[2][1] = true;

        assert_eq!(Piece::new(PieceType::L,m,point),piece);

        //third rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = false; m.data[0][2]=true;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2]=true;

        assert_eq!(Piece::new(PieceType::L,m,point),piece);

        //back to start rotation

        piece.perform_clockwise_rotation();

        assert_eq!(get_l_piece(),piece);
    }


    #[test]
    fn rotate_clockwise_o(){
        let mut piece = get_o_piece();

        //first rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m,point),piece);

        //second rotation
        piece.perform_clockwise_rotation();
        let point = Point::new(0,0);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m,point),piece);

        //third rotation
        piece.perform_clockwise_rotation();
        let point = Point::new(0,1);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m,point),piece);

        //back to start rotation
        piece.perform_clockwise_rotation();

        assert_eq!(get_o_piece(),piece);

    }

    #[test]
    fn rotate_clockwise_s(){
        let mut piece = get_s_piece();

        //first rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = false; m.data[2][1]= true;

        assert_eq!(Piece::new(PieceType::S,m,point),piece);

        //second rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = true; m.data[0][2] = true;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2] = false;

        assert_eq!(Piece::new(PieceType::S,m,point),piece);

        //third rotation
        piece.perform_clockwise_rotation();
        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = false; m.data[2][1]= true;

        assert_eq!(Piece::new(PieceType::S,m,point),piece);

        //back to the start rotation
        piece.perform_clockwise_rotation();

        assert_eq!(get_s_piece(),piece);
    }

    #[test]
    fn rotate_clockwise_t(){
        let mut piece = get_t_piece();

        //first rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = false; m.data[2][1] = true;

        assert_eq!(Piece::new(PieceType::T,m,point),piece);
        //second rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = true; m.data[0][2] = false;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2] = true;

        assert_eq!(Piece::new(PieceType::T,m,point),piece);

        //third rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = true; m.data[2][1] = false;

        assert_eq!(Piece::new(PieceType::T,m,point),piece);

        //back to start rotation
        piece.perform_clockwise_rotation();

        assert_eq!(get_t_piece(),piece);
    }



    #[test]
    fn rotate_clockwise_z(){
        let mut piece = get_z_piece();

        //first rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0]= true; m.data[2][1]=false;

        assert_eq!(Piece::new(PieceType::Z,m,point),piece);

        //second rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2] = false;
        m.data[1][0] = false; m.data[1][1] = true; m.data[1][2] = true;

        assert_eq!(Piece::new(PieceType::Z,m,point),piece);

        //third rotation
        piece.perform_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0]= true; m.data[2][1]=false;

        assert_eq!(Piece::new(PieceType::Z,m,point),piece);

        //back to start rotation
        piece.perform_clockwise_rotation();

        assert_eq!(get_z_piece(),piece);
    }

    #[test]
    fn rotate_counterclockwise_i(){
        let mut piece = get_i_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(1, 4);
        m.data[0][0] = true; m.data[0][1]=true; m.data[0][2]=true; m.data[0][3]=true;

        assert_eq!(Piece::new(PieceType::I,m, point),piece);

        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(2, 0);
        let mut m = Matrix::new(4, 1);
        m.data[0][0] = true;
        m.data[1][0] = true;
        m.data[2][0] = true;
        m.data[3][0] = true;

        assert_eq!(Piece::new(PieceType::I,m, point),piece);


        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0, 2);
        let mut m = Matrix::new(1, 4);
        m.data[0][0] = true; m.data[0][1]=true; m.data[0][2]=true; m.data[0][3]=true;


        assert_eq!(Piece::new(PieceType::I,m, point),piece);

        //back to start rotation
        piece.perform_counter_clockwise_rotation();
        assert_eq!(get_i_piece(),piece);
    }

    #[test]
    fn rotate_counterclockwise_j(){
        let mut piece = get_j_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2]=true;
        m.data[1][0] = false; m.data[1][1] = false; m.data[1][2]=true;

        assert_eq!(Piece::new(PieceType::J,m,point),piece);

        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = false;
        m.data[2][0] = true;  m.data[2][1] = false;

        assert_eq!(Piece::new(PieceType::J,m,point),piece);

        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = false; m.data[0][2]=false;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2]=true;



        assert_eq!(Piece::new(PieceType::J,m,point),piece);

        //back to start rotation
        piece.perform_counter_clockwise_rotation();

        assert_eq!(get_j_piece(),piece);
    }

    #[test]
    fn rotate_counterclockwise_l(){
        let mut piece = get_l_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = false; m.data[0][2]=true;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2]=true;

        assert_eq!(Piece::new(PieceType::L,m,point),piece);

        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = false; m.data[1][1] = true;
        m.data[2][0] = false;  m.data[2][1] = true;

        assert_eq!(Piece::new(PieceType::L,m,point),piece);

        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2]=true;
        m.data[1][0] = true; m.data[1][1] = false; m.data[1][2]=false;

        assert_eq!(Piece::new(PieceType::L,m,point),piece);

        //back to start rotation

        piece.perform_counter_clockwise_rotation();

        assert_eq!(get_l_piece(),piece);
    }


    #[test]
    fn rotate_counterclockwise_o(){
        let mut piece = get_o_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0,1);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m,point),piece);

        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0,0);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m,point),piece);

        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(2, 2);
        m.data[0][0] = true; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;

        assert_eq!(Piece::new(PieceType::O,m,point),piece);

        //back to start rotation
        piece.perform_counter_clockwise_rotation();

        assert_eq!(get_o_piece(),piece);

    }


    #[test]
    fn rotate_counterclockwise_s(){
        let mut piece = get_s_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = false; m.data[2][1]= true;

        assert_eq!(Piece::new(PieceType::S,m,point),piece);

        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = true; m.data[0][2] = true;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2] = false;

        assert_eq!(Piece::new(PieceType::S,m,point),piece);

        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = false; m.data[2][1]= true;

        assert_eq!(Piece::new(PieceType::S,m,point),piece);

        //back to the start rotation
        piece.perform_counter_clockwise_rotation();

        assert_eq!(get_s_piece(),piece);
    }

    #[test]
    fn rotate_counterclockwise_t(){
        let mut piece = get_t_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = true; m.data[0][1] = false;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = true; m.data[2][1] = false;

        assert_eq!(Piece::new(PieceType::T,m,point),piece);
        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = false; m.data[0][1] = true; m.data[0][2] = false;
        m.data[1][0] = true; m.data[1][1] = true; m.data[1][2] = true;

        assert_eq!(Piece::new(PieceType::T,m,point),piece);

        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0] = false; m.data[2][1] = true;

        assert_eq!(Piece::new(PieceType::T,m,point),piece);

        //back to start rotation
        piece.perform_counter_clockwise_rotation();

        assert_eq!(get_t_piece(),piece);
    }


    #[test]
    fn rotate_counterclockwise_z(){
        let mut piece = get_z_piece();

        //first rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 1);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0]= true; m.data[2][1]=false;

        assert_eq!(Piece::new(PieceType::Z,m,point),piece);

        //second rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(0, 1);
        let mut m = Matrix::new(2, 3);
        m.data[0][0] = true; m.data[0][1] = true; m.data[0][2] = false;
        m.data[1][0] = false; m.data[1][1] = true; m.data[1][2] = true;

        assert_eq!(Piece::new(PieceType::Z,m,point),piece);

        //third rotation
        piece.perform_counter_clockwise_rotation();

        let point = Point::new(1, 0);
        let mut m = Matrix::new(3, 2);
        m.data[0][0] = false; m.data[0][1] = true;
        m.data[1][0] = true; m.data[1][1] = true;
        m.data[2][0]= true; m.data[2][1]=false;

        assert_eq!(Piece::new(PieceType::Z,m,point),piece);

        //back to start rotation
        piece.perform_counter_clockwise_rotation();

        assert_eq!(get_z_piece(),piece);
    }

}