use chomp_ai::Chomp;
use std::env;
use std::io::{self, Write};

fn main() {
    // let args: Vec<String> = env::args().collect();
    let mut board = Chomp::new(3, 2).unwrap();

    //if args.len() < 3 {
    //    todo!("Error msg");
    //}

    // validate that the args are usize

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
