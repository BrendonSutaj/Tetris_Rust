// Make this struct cloneable. printable and comparable.
// Represents a boolean Matrix as a Vec<Vec<bool>> with rows, columns.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<Vec<bool>>,
}


impl Matrix {
    // Initialize the Matrix with false values.
    pub fn new(rows: usize, columns: usize) -> Matrix {
        Matrix {
            rows,
            columns,
            data: vec![vec![false; columns]; rows]
        }
    }


    ///
    /// This function is used to rotate the matrix clockwise.
    ///
    pub fn rotate_clockwise(&mut self) {
        let rows = self.rows;
        let columns = self.columns;
        // Create a new matrix.
        let mut new_matrix = Matrix::new(columns, rows);

        // Rotate the entries.
        for x in 0..rows {
            for y in 0..columns {
                new_matrix.data[y][rows - 1 - x] = self.data[x][y].clone();
            }
        }

        // Change rows and columns and set the new matrix data.
        self.rows = new_matrix.rows;
        self.columns = new_matrix.columns;
        self.data = new_matrix.data;
    }


    ///
    /// This function performs a counter clockwise rotation of the matrix.
    ///
    pub fn rotate_counter_clockwise(&mut self) {
        let rows = self.rows;
        let columns = self.columns;
        // Create a new matrix.
        let mut new_matrix = Matrix::new(columns, rows);

        // Rotate the entries of the matrix.
        for x in 0..rows {
            for y in 0..columns {
                new_matrix.data[columns - 1 - y][x] = self.data[x][y].clone();
            }
        }

        // Change the rows, columns and set the new data.
        self.rows = new_matrix.rows;
        self.columns = new_matrix.columns;
        self.data = new_matrix.data;
    }
}