// Make this struct cloneable, printable and comparable.
// This struct represents the possible Commands the autoplayer can compute.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Down,
    Left,
    Right,
    RotateClockWise,
}