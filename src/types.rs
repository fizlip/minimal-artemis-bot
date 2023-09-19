use mev_share::{sse};

#[derive(Debug, Clone)]
pub enum Event {
    MEVShareEvent(sse::Event),
}
#[derive(Debug, Clone)]
pub enum Action {
}
#[derive(Debug, Clone)]
pub struct Config {}
