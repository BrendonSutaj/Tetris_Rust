// Make this struct copyable, cloneable. printable and comparable.
// Represents a simple 2d point with usize type values.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x_coordinate: usize,
    pub y_coordinate: usize,
}


impl Point {
    pub fn new(x_coordinate: usize, y_coordinate: usize) -> Point {
        Point {
            x_coordinate,
            y_coordinate
        }
    }


    ///
    /// This function is just used to create an empty_point.(with stupid high values to prevent the annoying "subtract with overflow error")
    ///
    pub fn empty_point() -> Point {
        Point::new(1000000, 1000000)
    }


    ///
    /// This function is used to set a given point.
    ///
    pub fn set(&mut self, x_coordinate: usize, y_coordinate: usize) {
        self.x_coordinate = x_coordinate;
        self.y_coordinate = y_coordinate;
    }
}