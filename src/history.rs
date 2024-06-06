use crate::{bitboard::Bitboard, r#move::Move};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryItem {
    pub r#move: Move,
    pub squares_attacked: Bitboard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct History(pub Vec<HistoryItem>);

impl std::fmt::Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print all moves in algebraic notation
        for item in &self.0 {
            let _ = write!(f, "{} ", item.r#move);
        }
        Ok(())
    }
}

impl History {
    pub fn push(&mut self, item: HistoryItem) {
        self.0.push(item);
    }
    pub fn pop(&mut self) -> Option<HistoryItem> {
        self.0.pop()
    }
}
