use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use quote::spanned::Spanned;

pub trait SpannedItemSpan = Spanned + Debug + Clone;
pub trait SpannedItemItem = Hash + Eq + PartialEq + Debug + Clone + Display + Default;

#[derive(Debug, Clone, Eq)]
pub struct SpannedItem<Span: SpannedItemSpan, Item: SpannedItemItem>(pub Option<Span>, pub Item);

impl<Span: SpannedItemSpan, Item: SpannedItemItem> SpannedItem<Span, Item> {
    pub fn new_empty(item: Item) -> Self {
        Self(None, item)
    }
    pub fn new(span: Span, item: Item) -> Self {
        Self(Some(span), item)
    }
}

impl<Span: SpannedItemSpan, Item: SpannedItemItem> Hash for SpannedItem<Span, Item> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}
impl<Span: SpannedItemSpan, Item: SpannedItemItem> PartialEq for SpannedItem<Span, Item> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<Span: SpannedItemSpan, Item: SpannedItemItem> Display for SpannedItem<Span, Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl<Span: SpannedItemSpan, Item: SpannedItemItem> Default for SpannedItem<Span, Item> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
