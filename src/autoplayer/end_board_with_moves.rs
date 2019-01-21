//Imports
use Board;
use Command;

// This struct is used to store the board computed by the commands in commands.
#[derive(Debug, Clone)]
pub struct EndBoardWithMoves {
    pub board: Board,
    pub commands: Vec<Command>,
}


impl EndBoardWithMoves {
    pub fn new(board: Board, commands: Vec<Command>) -> EndBoardWithMoves {
        EndBoardWithMoves{
            board,
            commands
        }
    }
}