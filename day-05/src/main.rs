use std::{fs::File, io::Read};

type StackList = Vec<Vec<String>>;
type Instruction = (usize, usize, usize);

fn main() {
    let mut file_contents = String::new();

    // Read the challenge input
    let mut file = File::open("input.txt").expect("Failed to open input file");
    file.read_to_string(&mut file_contents).expect("Failed to read input file");


    // Parse the input
    let mut parts = file_contents.split("\n\n");
    
    let mut crates_9000: StackList = parse_stacks(parts.next().unwrap());
    let mut crates_9001: StackList = crates_9000.clone();

    let instructions: Vec<Instruction> = parse_instructions(parts.next().unwrap());


    // Perform the instructions
    // NB: code could be optimised by merging instruction parsing and performance but this is more readable
    for instruction in instructions.iter() {
        crate_mover_9000(&mut crates_9000, instruction);
        crate_mover_9001(&mut crates_9001, instruction);
    }


    // Get end result
    let get_result = |crates: &StackList| -> String {
        crates.iter().map(|stack| stack.last().unwrap().as_str()).collect()
    };

    println!("End result with the CrateMover9000: {}", get_result(&crates_9000));
    println!("End result with the CrateMover9001: {}", get_result(&crates_9001));
}



/* DIFFERENT METHODS FOR MOVING CRATES */

fn crate_mover_9000(crates: &mut StackList, instruction: &Instruction) {
    let (from, to, amount) = *instruction;

    // Moving one crate at a time
    for _ in 0..amount {
        if let Some(to_add) = crates[from].pop() {
            crates[to].push(to_add);
        }
    }
}

fn crate_mover_9001(crates: &mut StackList, instruction: &Instruction) {
    let (from, to, amount) = *instruction;

    // Here all the crates are moved as one
    let mut temp_stack: Vec<String> = Vec::new();
    for _ in 0..amount {
        if let Some(to_add) = crates[from].pop() {
            temp_stack.push(to_add);
        }
    }

    temp_stack.reverse();
    crates[to].append(&mut temp_stack);
}



/* PARSING THE DIFFERENT STACKS (HARDEST PART BY FAR) */

fn parse_stacks(text: &str) -> StackList {
    // Parse each line
    let mut vertical_splits: StackList = text
        .lines()
        .map(parse_line)
        .collect();

    // Discard the last one and reverse the list to get the crates in order to perform vertical to horizontal switch
    let _ = vertical_splits.pop();
    vertical_splits.reverse();

    // Vertical to horizontal switch (bit scuffed but works)
    let mut horizontal_splits: StackList = Vec::new();
    vertical_splits.iter().for_each(|row| {
        for i in 0..row.len() {
            if horizontal_splits.get(i) == None {
                horizontal_splits.push(Vec::new());
            }

            if row[i] != " " {
                horizontal_splits[i].push(String::from(row[i].as_str()));
            }
        }
    });

    horizontal_splits
}

fn parse_line(str: &str) -> Vec<String> {
    let mut splits: Vec<String> = Vec::new();

    let mut i = 0;
    let mut chars = str.chars();

    // Basically splits each line in sections of four and only keeps the second character each time
    while let Some(c) = chars.next() {
        if i == 1 {
            splits.push(c.to_string());
        }
        i = (i+1) % 4;
    }

    splits
}



/* INSTRUCTION PARSING */

fn parse_instructions(text: &str) -> Vec<Instruction> {
    text.lines().map(|line| {
        // Splits the string by words and only keeps the numbers
        let parts: Vec<&str> = line.split(" ").collect();

        // Get an number from the string
        // NB: Bit of a sketchy line but this isn't production worthy code anyway
        let parse = |index: usize| parts.get(index).unwrap().parse::<usize>().unwrap(); 

        // Declaring each variable is more verbose but more readable
        let from = parse(3) - 1;
        let to = parse(5) - 1;
        let amount = parse(1);

        (from, to, amount)
    }).collect()
}