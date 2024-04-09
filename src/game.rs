trait Game {
    fn drop_piece(&self, board: &mut Vec<Vec<char>>, col: usize, piece: char) -> bool;
}