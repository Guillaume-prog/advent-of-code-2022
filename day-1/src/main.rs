use std::{fs::File, io::Read};

fn main() {
    // Extract the contents of the file
    let mut file_contents = String::new();

    let mut f = File::open("input.txt").expect("file not found");
    f.read_to_string(&mut file_contents).expect("something went wrong reading the file");

    // Separate different groups (separated by a blank line)
    let groups: Vec<&str> = file_contents.split("\n\n").collect();
    
    // Get the amount of calories each elf has
    let mut calorie_sums: Vec<i32> = groups.iter().map(get_sum_of_calories).collect();
    calorie_sums.sort();
    calorie_sums.reverse();

    // Solution to part 1 : Get the amount of calories the top elf has
    let best_elf_calories = calorie_sums[0];
    println!("The elf with the most calories has {} calories", best_elf_calories);

    // Solution to part 2 : Get the sum of the top 3 elves
    let top_3_elves_calories = calorie_sums[0] + calorie_sums[1] + calorie_sums[2];
    println!("The top 3 elves have a combined total of {} calories", top_3_elves_calories);
}

fn get_sum_of_calories(group: &&str) -> i32 {
    let calories: Vec<i32> = group
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    calories.iter().sum()
}