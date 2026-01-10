pub mod board;
pub mod piece;

#[cfg(test)]
mod tests {

    use crate::board::Board;

    #[test]
    fn test() {
        let board = Board::new();
        let mut output = String::new();

        // dbg the board

        println!("{}", output);
    }
}
