use std::{fs::File, io::Read};

fn main() {
    let mut file_contents = String::new();

    // Read the challenge input
    let mut file = File::open("input.txt").expect("Failed to open input file");
    file.read_to_string(&mut file_contents).expect("Failed to read input file");

    // Get score of each rucksack and calculate the sum
    let priority_total: u32 = file_contents
        .lines()
        .map(get_rucksack_score)
        .sum();

    // Get score for each group
    let group_scores_sum = get_group_scores(&file_contents);

    println!("Total priority: {}", priority_total);
    println!("Total group score: {}", group_scores_sum);
}

fn get_rucksack_score(rucksack: &str) -> u32 {
    // Get both compartments (split line in two equal parts)
    let compartment1 = rucksack[0..(rucksack.len() / 2)].to_string();
    let compartment2 = rucksack[(rucksack.len() / 2)..rucksack.len()].to_string();

    // Get the common letter between the two compartments (we assume there's only one)
    let common_letter = compartment1
        .chars()
        .filter(|letter| compartment2.contains(letter.to_string().as_str()))
        .next()
        .unwrap();

    get_letter_score(common_letter)
}

fn get_group_scores(file_contents: &String) -> u32 {
    let mut i = 0;
    let mut cur_group = [""; 3];
    let mut group_scores_sum: u32 = 0;

    // Split the text into groups of 3 lines
    for line in file_contents.lines() {
        cur_group[i] = line;
        i += 1;

        if i == 3 {
            i = 0;

            // Get the common letter between the three rucksacks (we assume there's only one)
            let common_letter = cur_group[0]
            .chars()
            .filter(|letter| {
                let has = |i: usize| cur_group[i].contains(letter.to_string().as_str());
                has(1) && has(2)
            })
            .next()
            .unwrap();

            group_scores_sum += get_letter_score(common_letter)
        }
    }
    
    group_scores_sum
}

fn get_letter_score(letter: char) -> u32 {
    let mut letter_score = letter as u32;

    if letter_score >= 97 && letter_score <= 122 {
        // Lowercase letter (97-122 -> 1-26)
        letter_score -= 96;
    } else {
        // Uppercase letter (65-90 -> 27-52)
        letter_score = letter_score - 64 + 26;
    }

    letter_score
}