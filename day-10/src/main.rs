fn main() {
    let input = include_str!("../input.txt");
    let values = get_xreg_values(input);

    // Step 2
    draw_crt(&values, 40);

    // Step 1
    let signal_strengh_total = calculate_signal_strengh(&values, false);
    println!("Signal strength total: {}", signal_strengh_total);
}

// Step 2 solution
fn draw_crt(values: &Vec<i32>, width: usize) {
    for (cycle, xreg) in values.iter().enumerate() {
        if cycle > 0 && cycle % width == 0 { print!("\n"); }
        
        let pixel = (cycle % width) as i32;
        let is_drawing_sprite = pixel >= (xreg - 1) && pixel <= (xreg + 1);

        if is_drawing_sprite { print!("⬛"); }
        else                 { print!("⬜"); }
    }
    print!("\n");
}

// Step 1 solution
fn calculate_signal_strengh(values: &Vec<i32>, verbose: bool) -> i32 {
    let important_cycles = vec![20, 60, 100, 140, 180, 220];
    
    let signal_strengh_sum: i32 = important_cycles.iter().map(|cycle| {
        let cycle_index = (*cycle as usize) - 1;
        let xreg = values.get(cycle_index).unwrap();
        let signal_strengh: i32 = cycle * xreg;

        if verbose { println!("cycle: {}, x: {}, signal_strengh: {}", cycle, xreg, signal_strengh); }
        signal_strengh
    }).sum();

    signal_strengh_sum
}

fn get_xreg_values(input: &str) -> Vec<i32> {
    let mut xreg = 1;
    let mut values: Vec<i32> = Vec::new();

    for line in input.lines() {
        values.push(xreg); // regardless of "addx" or "noop" operations

        // if line is "addx v"
        if let Some((_, val)) = line.split_once(" ") {
            values.push(xreg);
            xreg += val.parse::<i32>().unwrap();
        }
    }

    values
}