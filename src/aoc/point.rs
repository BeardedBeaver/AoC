#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<I> {
    pub row: I,
    pub col: I,
}
