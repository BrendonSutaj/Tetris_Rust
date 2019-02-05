// Make this struct cloneable, printable and comparable.
// Represents the different tetris piece_types.
#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
    None,
}