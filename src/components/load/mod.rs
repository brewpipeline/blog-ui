pub mod item;
pub mod list;

#[derive(Clone, PartialEq)]
pub enum ExternalError<E: Clone + PartialEq> {
    Net(String),
    Content(E),
}
