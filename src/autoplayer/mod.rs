pub mod commands;
mod end_board_with_moves;

// Imports.
use game::Game;
use self::commands::Command;
use board::Board;
use game::move_direction::MoveDirection;
use pieces::piece_type::PieceType;
use self::end_board_with_moves::EndBoardWithMoves;


// Struct representing the Autoplayer.
// Make it printable and cloneable.
#[derive(Debug, Clone)]
pub struct Autoplayer {
    pub commands: Vec<Command>,
}


impl Autoplayer {
    pub fn new() -> Autoplayer {
        Autoplayer {
            commands: vec![],
        }
    }


    ///
    /// This function just returns the next command or Command::Down if there is no commands to perform.
    ///
    pub fn perform_move(&mut self, _game: &mut Game) -> Command {
        if self.commands.len() != 0 {
            self.commands.pop().unwrap()
        } else {
            Command::Down
        }
    }


    ///
    /// This function is used to compute the next commands.
    ///
    pub fn compute_move(&mut self, game: &mut Game) {
        let all_boards = self.get_all_boards(game);

        // Determine the board and commands with the maximum heuristic value.
        let mut max = std::f64::MIN;
        let mut best_moves = vec![];
        for x in all_boards.into_iter() {
            let sc = self.heuristic(&mut x.board.clone());

            if sc > max {
                max = sc;
                best_moves = x.commands;
            }
        }

        // Append these commands now.
        self.commands.append(best_moves.as_mut());
    }


    ///
    /// This function computes all possible boards and the commands needed to compute them.
    ///
    fn get_all_boards(&self, game: &Game) -> Vec<EndBoardWithMoves> {

        // Do the computations on a game_copy.
        let mut game_copy = game.clone();
        let mut all_boards: Vec<EndBoardWithMoves> = vec![];

        // The spawn position is at game.board.columns / 2.
        let mut piece_location = (game.board.columns / 2) + 1;

        // The commands needed for a computed board.
        let mut commands: Vec<Command> = vec![];

        // First compute the board that results by executing no command at all, and push the result into all_boards.
        all_boards.push(EndBoardWithMoves::new(self.drop(&game_copy), commands.clone()));

        // Now try all rotations and go left each iteration as long as it is possible to perform left move commands.
        while 0 < piece_location {
            for i in 0..4 {
                if game_copy.rotate_piece_clockwise() && i < 3 {
                    commands.push(Command::RotateClockWise);
                    all_boards.push(EndBoardWithMoves::new(self.drop(&game_copy), commands.clone()));
                }
            }
            // This function is used to remove every command other than Command::Left ones.
            commands.retain(|x| *x == Command::Left);

            // If we can move left once more push the command into commands.
            if game_copy.move_in_direction(MoveDirection::Left) {
                commands.push(Command::Left);
                all_boards.push(EndBoardWithMoves::new(self.drop(&game_copy), commands.clone()));
            }

            piece_location -= 1;
        }

        // Now clear all commands and try the right moves.
        commands.clear();

        // Clone to reset the game.
        game_copy = game.clone();

        // Do the same as before but going to the right.
        game_copy.move_in_direction(MoveDirection::Right);
        commands.push(Command::Right);
        all_boards.push(EndBoardWithMoves::new(self.drop(&game_copy), commands.clone()));
        piece_location = (game.board.columns / 2) + 1;

        // Rotate it again and try all boards. One less iteration because the board starts with coordinate 0.
        while 1 < piece_location {
            for i in 0..4 {
                if game_copy.rotate_piece_clockwise() && i < 3 {
                    commands.push(Command::RotateClockWise);
                    all_boards.push(EndBoardWithMoves::new(self.drop(&game_copy), commands.clone()));
                }
            }

            // Get only the Command::Right commands, remove everything else.
            commands.retain(|x| *x == Command::Right);

            if game_copy.move_in_direction(MoveDirection::Right) {
                commands.push(Command::Right);
                all_boards.push(EndBoardWithMoves::new(self.drop(&game_copy), commands.clone()));
            } else {
                break;
            }
            piece_location -= 1;
        }

        all_boards
    }


    ///
    /// This function is used to compute the heuristics and evaluate the board given.
    /// -2.5 * sum of all heights // 8.0 * completed rows // -4.5 * number of holes // 4.2 * touching walls // 4.0 * touching pieces
    ///
    pub fn heuristic(&self, board: &mut Board) -> f64 {
        let mut sum_of_heights = 0.0;
        let mut completed_rows = 0.0;
        let mut number_of_holes = 0.0;
        let mut wall_touched = 0.0;

        // Compute the sum of heights, the number of holes and wall touched.
        let mut heights = vec![0.0; board.columns];
        let mut first_piece_found;
        for column in 0..board.columns {
            first_piece_found = false;
            for row in 0..board.rows {
                if (column == 0 || column == board.columns - 1) && board.board[row][column] != PieceType::None  {
                    wall_touched += 1.0;
                }
                if row == board.rows - 1 && board.board[row][column] != PieceType::None  {
                    wall_touched += 2.0;
                }
                if board.board[row][column] == PieceType::None {
                    if first_piece_found {
                        number_of_holes = number_of_holes + 1.0;
                    }
                } else {
                    if !first_piece_found {
                        heights[column] = board.rows as f64 - row as f64;
                    }
                    first_piece_found = true;
                }
            }
        }

        // Get the sum of heights.
        for height in heights {
            sum_of_heights += height;
        }

        let mut row_completed;

        // Compute the number of touching pieces.
        let mut touching_pieces_score = 0.0;
        for x in 0..board.rows {
            row_completed = true;
            for y in 0..board.columns {
                if board.board[x][y] != PieceType::None {
                    if x < (board.rows - 1) {
                        if board.board[x + 1][y] != PieceType::None {
                            touching_pieces_score += 1.0;
                        }
                    }
                    if x > 0 {
                        if board.board[x - 1][y] != PieceType::None {
                            touching_pieces_score += 1.0;
                        }
                    }
                    if y < (board.columns - 1) {
                        if board.board[x][y + 1] != PieceType::None {
                            touching_pieces_score += 1.0;
                        }
                    }
                    if y > 0 {
                        if board.board[x][y - 1] != PieceType::None {
                            touching_pieces_score += 1.0;
                        }
                    }
                } else {
                    row_completed = false;
                }
            }
            if row_completed {
                completed_rows += 1.0;
            }
        }



        sum_of_heights * -2.50 + completed_rows * 8.0 + number_of_holes * -4.50 + wall_touched * 4.20 + touching_pieces_score * 4.0
    }


    ///
    /// This function is used to compute the out coming board by dropping the actual piece.
    ///
    pub fn drop(&self, game: &Game) -> Board {
        let mut game_copy = game.clone();

        while game_copy.move_in_direction(MoveDirection::Down) {}

        game_copy.board
    }
}

