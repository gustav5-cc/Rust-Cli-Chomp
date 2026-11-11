use chomp_ai::Chomp;
use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <rows> <cols>", args[0]);
        process::exit(1);
    }

    let rows: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: <rows> must be a valid positive number.");
            process::exit(1);
        }
    };

    let cols: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: <cols> must be a valid positive number.");
            process::exit(1);
        }
    };

    //let mut board = Chomp::new(rows, cols).unwrap();
    let mut board = Chomp::new(rows, cols).unwrap_or_else(|e| {
        eprintln!(
            "Failed to initialize the board, {}. \nChoose a board between 1-4 rows and 1-5 cols",
            e
        );
        process::exit(1);
    });

    // print board
    board.print_board();

    // loop until game is done
    while !board.poison_eaten() {
        // let user make a move
        // was it a valid move
        // if not recover

        loop {
            let (x, y) = get_player_move();

            match board.make_move(x, y) {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    println!("{}. Please try again.", e);
                }
            }
        }

        // print board
        board.print_board();

        // If the user makes a move and no winning positions are available the ai lost
        if board.poison_eaten() {
            println!("You lost, the ai won!");
            break;
        }

        // let ai make a move
        if let Some((row, col)) = board.winning_move() {
            let _ = board.make_move(row, col);
            println!("Computer: {row} {col}")
        } else {
            // Eat any uneatean square or poison
            match board.fallback_move() {
                Some((row, col)) => println!("Computer: {row} {col}"),
                None => println!("Something went wrong"),
            }
        }

        // print board
        board.print_board();

        // If no winning position is available after ai acts, human lost
        if board.poison_eaten() {
            println!("The ai lost, you won!");
            break;
        }
    }
}

pub fn get_player_move() -> (usize, usize) {
    let mut input = String::new();

    loop {
        input.clear();

        print!("Make a move (row col): ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() == 2
            && let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>())
        {
            return (row, col);
        }

        println!("Invalid input. Please enter two positive numbers separated by a space.");
    }
}
