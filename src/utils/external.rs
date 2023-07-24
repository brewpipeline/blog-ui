pub trait ExternalResultContainer {
    type Inner;
    type Error;
    fn result(self) -> Result<Self::Inner, Self::Error>;
}

pub trait Identifiable {
    type Id: Eq + std::hash::Hash;
    fn id(&self) -> &Self::Id;
}

impl Identifiable for () {
    type Id = Self;
    fn id(&self) -> &Self::Id {
        self
    }
}

pub trait ExternalItemContainer {
    type Item;
    fn item(self) -> Self::Item;
}

#[derive(Clone, PartialEq)]
pub struct ExternalListContainerParams<P: Clone + PartialEq> {
    pub params: P,
    pub limit: u64,
    pub skip: u64,
}

pub trait ExternalListContainer {
    type Item;
    fn items(self) -> Vec<Self::Item>;
    fn total(&self) -> u64;
    fn skip(&self) -> u64;
    fn limit(&self) -> u64;
}
