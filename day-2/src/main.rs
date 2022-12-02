mod shape;
use shape::{Shape, Outcome};

use std::{fs::File, io::Read};

enum Method {
    Method1, Method2
}

fn main() {
    println!("Using the method 1");
    solve_challenge(Method::Method1);

    println!("\nUsing the method 2");
    solve_challenge(Method::Method2);
}

fn solve_challenge(method: Method) {
    let rounds = parse_input("input.txt", method);
    let total_score: i32 = rounds
        .iter()
        .map(|(m, o)| compute_round_score(m, o))
        .sum();

    println!("Total score: {}", total_score);
}

fn compute_round_score(my_move: &Shape, opponent_move: &Shape) -> i32 {
    let shape_score = match my_move {
        Shape::Rock     => 1,
        Shape::Paper    => 2,
        Shape::Scissors => 3
    };

    let outcome_score = match my_move.outcome_against(opponent_move) {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win  => 6
    };

    shape_score + outcome_score
}

fn parse_input(path: &str, method: Method) -> Vec<(Shape, Shape)> {
    let mut payload = String::new();

    // Start by reading file and getting each line
    let mut input_file = File::open(path).expect("Couldn't open input file");
    input_file.read_to_string(&mut payload).expect("Failed to read input file");

    // Method that converts a line into a pair of shapes
    let get_tokens = |line: &str| {
        let tokens: Vec<&str> = line.split(" ").collect();

        // Opponent mapping will always be the same
        let opponent_shape = match tokens[0] {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }.unwrap();

        // Get my shape based on strategy used
        let my_shape = get_my_shape(&method, tokens[1], &opponent_shape);

        // Return the pair of shapes
        (my_shape, opponent_shape)
    };

    // Run and return conversion
    payload.lines().map(get_tokens).collect()
}

fn get_my_shape(method: &Method, my_move_encrypted: &str, opponent_move: &Shape) -> Shape {
    match method {
        Method::Method1 => match my_move_encrypted {
            "X" => Some(Shape::Rock),
            "Y" => Some(Shape::Paper),
            "Z" => Some(Shape::Scissors),
            _ => None
        },
        Method::Method2 => match my_move_encrypted {
            "X" => Some(opponent_move.wins_against()), // Need to lose
            "Y" => Some(*opponent_move),               // Need to draw
            "Z" => Some(opponent_move.loses_to()),     // Need to win
            _ => None
        }
    }.unwrap()
}
