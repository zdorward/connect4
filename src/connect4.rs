use rand::Rng; // For bot moves
use std::io;

const ROWS: usize = 6;
const COLS: usize = 7;

pub fn main() {
    let mut board: Vec<Vec<char>> = vec![vec!['.'; COLS]; ROWS];
    let mut game_over = false;
    
    while !game_over {
        print_board(&board);
        
        // Player's turn
        let player_col = get_player_input();
        if drop_piece(&mut board, player_col, 'O') {
            if check_win(&board, 'O') {
                game_over = true;
                println!("Player wins!");
                break;
            }
        }

        // Bot's turn
        let bot_col = rand::thread_rng().gen_range(0..COLS);
        println!("Bot chooses column {}", bot_col + 1); // Add 1 to match user-facing numbering
        if drop_piece(&mut board, bot_col, 'X') {
            if check_win(&board, 'X') {
                game_over = true;
                println!("Bot wins!");
                break;
            }
        }
    }
    
    print_board(&board);
}

pub fn print_board(board: &Vec<Vec<char>>) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    println!("1 2 3 4 5 6 7"); // Start from 1 instead of 0
}

pub fn get_player_input() -> usize {
    loop {
        println!("Enter a column number (1-7):"); // Prompt for 1-7
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= COLS => return num - 1, // Convert to 0-based indexing
            _ => println!("Invalid input, please try again."),
        }
    }
}

pub fn drop_piece(board: &mut Vec<Vec<char>>, col: usize, piece: char) -> bool {
    for row in board.iter_mut().rev() {
        if row[col] == '.' {
            row[col] = piece;
            return true;
        }
    }
    println!("Column is full, try a different one.");
    false
}

pub fn check_win(board: &Vec<Vec<char>>, piece: char) -> bool {
    // Horizontal check
    for row in 0..ROWS {
        for col in 0..=COLS - 4 {
            if board[row][col] == piece && board[row][col + 1] == piece && board[row][col + 2] == piece && board[row][col + 3] == piece {
                return true;
            }
        }
    }

    // Vertical check
    for col in 0..COLS {
        for row in 0..=ROWS - 4 {
            if board[row][col] == piece && board[row + 1][col] == piece && board[row + 2][col] == piece && board[row + 3][col] == piece {
                return true;
            }
        }
    }

    // Diagonal (ascending) check
    for col in 0..=COLS - 4 {
        for row in 3..ROWS {
            if board[row][col] == piece && board[row - 1][col + 1] == piece && board[row - 2][col + 2] == piece && board[row - 3][col + 3] == piece {
                return true;
            }
        }
    }

    // Diagonal (descending) check
    for col in 0..=COLS - 4 {
        for row in 0..=ROWS - 4 {
            if board[row][col] == piece && board[row + 1][col + 1] == piece && board[row + 2][col + 2] == piece && board[row + 3][col + 3] == piece {
                return true;
            }
        }
    }

    false
}
