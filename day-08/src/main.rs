use std::{fs::File, io::Read};

mod matrix;
use matrix::Matrix;

mod iter;
use iter::Iter2D;

struct Tree {
    height: u8,
    neighbours: Vec<Vec<u8>>
}

fn main() {
    let matrix = parse_input("input.txt");

    let mut total_visible_trees = 0;
    let mut heighest_scenic_score: u32 = 0;

    let (width, height) = matrix.size();
    for (x, y) in Iter2D::new(width, height) {
        let tree = generate_tree(&matrix, x, y);
        
        if is_visible(&tree) {
            total_visible_trees += 1;
        }

        let score = get_scenic_score(&tree);
        if score > heighest_scenic_score {
            heighest_scenic_score = score;
        }
    }

    println!("Total visible trees from outside: {}", total_visible_trees);
    println!("Scenic score of ideal tree for treehouse: {}", heighest_scenic_score);
}

fn is_visible(tree: &Tree) -> bool {
    for neighbour in &tree.neighbours {
        // We can see the tree if its at the edge of the forest
        if neighbour.is_empty() {
            return true;
        }

        // If our tree is the tallest in a direction, we can see it
        if tree.height > *neighbour.iter().max().unwrap() {
            return true;
        }
    }

    false
}

fn get_scenic_score(tree: &Tree) -> u32 {
    let mut scenic_score: u32 = 1;

    for neighbour in &tree.neighbours {
        // An edge tree is rubbish as it isn't very hidden
        if neighbour.is_empty() {
            scenic_score = 0;
            continue;
        }

        // Keep counting until you can't see past a tree
        let mut dir_score = 0;
        for tree_height in neighbour {
            dir_score += 1;
            if tree_height >= &tree.height {
                break;
            }
        }

        scenic_score *= dir_score;
    }

    scenic_score
}

fn generate_tree(matrix: &Matrix, x: usize, y: usize) -> Tree {
    let height = matrix.get(x, y);
    let col = matrix.get_col(x);
    let row = matrix.get_row(y);

    let mut ranges: Vec<Vec<u8>> = Vec::new();
    let mut add = |list: Vec<&u8>| ranges.push(list.iter().map(|v| **v).collect());
    
    // Get the score of all the surrounding trees
    let mut left = row[..x].to_vec();
    left.reverse();
    add(left);

    let right = row[x+1..].to_vec();
    add(right);

    let mut top = col[..y].to_vec();
    top.reverse();
    add(top);

    let bottom = col[y+1..].to_vec();
    add(bottom);

    // Return the tree
    Tree { height: *height, neighbours: ranges }
}

fn parse_input(path: &str) -> Matrix {

    let mut input = String::new();

    // Read the challenge input
    let mut file = File::open(path).expect("Failed to open input file");
    file.read_to_string(&mut input).expect("Failed to read input file");

    let lines: Vec<&str> = input.lines().collect();

    // Get size of matrix
    let width = lines[0].len();
    let height = lines.len();

    let mut matrix = Matrix::new(width, height);

    // Get each character, convert to number, and insert into matrix
    for (x, y) in Iter2D::new(width, height) {
        let value_str = lines[y].chars().nth(x).unwrap().to_string();
        let value: u8 = value_str.parse().unwrap();

        matrix.set(x, y, value);
    }

    matrix
}
