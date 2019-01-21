// Make this struct cloneable, printable and comparable.
// Represents the possible MoveDirections.
#[derive(Debug, Clone, PartialEq)]
pub enum MoveDirection {
    Down,
    Left,
    Right
}