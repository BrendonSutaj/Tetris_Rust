pub mod matrix;
pub mod point;

use Game;

///
/// This module represents a utility module. Functions that are used throughout the game but don't belong anywhere else.
///

///
/// This function returns the time as f64 that it takes until the next automatic move_down of the tetris game occurs.
/// Values of the original NES Tetris were used here.
///
pub fn duration_for_level(game: &Game) -> f64 {
    if game.rows < 10 {
        0.8
    } else if game.rows < 20 {
        0.72
    } else if game.rows < 30 {
        0.63
    } else if game.rows < 40 {
        0.55
    } else if game.rows < 50 {
        0.47
    } else if game.rows < 60 {
        0.38
    } else if game.rows < 70 {
        0.3
    } else if game.rows < 80 {
        0.22
    } else if game.rows < 90 {
        0.13
    } else if game.rows < 100 {
        0.1
    } else if game.rows < 130 {
        0.08
    } else if game.rows < 160 {
        0.07
    } else if game.rows < 190 {
        0.05
    } else if game.rows < 290 {
        0.03
    } else {
        0.02
    }
}


///
/// TESTS FOR THE UTILITY MODULES.
///
#[cfg(test)]
mod tests {
    use utility::point::Point;
    use utility::matrix::Matrix;

    #[test]
    fn point_creation_set() {
        let mut p = Point::new(1,0);
        assert_eq!(p.x_coordinate,1);
        assert_eq!(p.y_coordinate,0);
        p.set(5,10);
        assert_eq!(p.x_coordinate,5);
        assert_eq!(p.y_coordinate,10);
    }


    #[test]
    fn matrix_creation(){
        let mut m = Matrix::new(5,2);
        assert_eq!(m.rows,5);
        assert_eq!(m.columns,2);
        for x in 0..5 {
            for y in 0..2 {
                assert_eq!(m.data[x][y],false);
            }

        }
    }

    #[test]
    fn matrix_rotate_clockwise(){
        let mut m = Matrix::new(5,2);
        m.rotate_clockwise();
        assert_eq!(m.rows,2);
        assert_eq!(m.columns,5);
    }

    #[test]
    fn matrix_counter_clockwise(){
        let mut m = Matrix::new(5,2);
        m.rotate_counter_clockwise();
        assert_eq!(m.rows,2);
        assert_eq!(m.columns,5);
    }

}