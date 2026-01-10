#![allow(clippy::unusual_byte_groupings)]
pub mod board;
pub mod piece;

#[cfg(test)]
mod tests {

    use crate::board::Board;

    #[test]
    fn test() {
        let board = Board::new();
    }
}
