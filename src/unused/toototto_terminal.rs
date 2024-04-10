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
        let player_piece = get_player_piece(); // T or O
        let player_col = get_player_input();
        if drop_piece(&mut board, player_col, player_piece) {
            if check_win(&board) {
                game_over = true;
                println!("Player wins!");
                break;
            }
        }

        // Bot's turn
        let bot_piece = ['T', 'O'][rand::thread_rng().gen_range(0..2)]; // Randomly chooses T or O
        let bot_col = rand::thread_rng().gen_range(0..COLS);
        println!("Bot chooses column {} and piece {}", bot_col + 1, bot_piece); // Add 1 for user-facing numbering
        if drop_piece(&mut board, bot_col, bot_piece) {
            if check_win(&board) {
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

pub fn get_player_piece() -> char {
    loop {
        println!("Choose a piece (T/O):");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "T" => return 'T',
            "O" => return 'O',
            _ => println!("Invalid input, please choose T or O."),
        }
    }
}

pub fn get_player_input() -> usize {
    loop {
        println!("Enter a column number (1-7):");
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


pub fn check_win(board: &Vec<Vec<char>>) -> bool {
    // Check all horizontal lines
    for row in board {
        let line = row.iter().collect::<String>();
        if line.contains("TOOT") || line.contains("OTTO") {
            return true;
        }
    }

    // Check all vertical lines
    for col in 0..COLS {
        let mut line = String::new();
        for row in board {
            line.push(row[col]);
        }
        if line.contains("TOOT") || line.contains("OTTO") {
            return true;
        }
    }

    // Check diagonal lines (ascending and descending)
    for col in 0..COLS {
        for row in 0..ROWS {
            // Ascending diagonal
            let mut ascend_line = String::new();
            let mut r = row;
            let mut c = col;
            while r < ROWS && c < COLS {
                ascend_line.push(board[r][c]);
                r += 1;
                c += 1;
            }
            if ascend_line.contains("TOOT") || ascend_line.contains("OTTO") {
                return true;
            }
            
            // Descending diagonal
            let mut descend_line = String::new();
            let mut r = row;
            let mut c = col;
            while r < ROWS && c < COLS && r >= 0 {
                descend_line.push(board[r][c]);
                if r == 0 { break; } // Prevent underflow
                r -= 1;
                c += 1;
            }
            if descend_line.contains("TOOT") || descend_line.contains("OTTO") {
                return true;
            }
        }
    }

    false
}

