#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionKind {
    Add(usize),
    Del(usize),
    ChangeOccupation,
    ChangeDescription,
    Rotate,
    Reset,
}
