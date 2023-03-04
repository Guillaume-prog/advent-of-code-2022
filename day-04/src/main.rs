use std::{fs::File, io::Read};

mod range;
use range::Range;

fn main() {
    let mut file_contents = String::new();

    // Read the challenge input
    let mut file = File::open("input.txt").expect("Failed to open input file");
    file.read_to_string(&mut file_contents).expect("Failed to read input file");

    // get pairs of ranges
    let mut total_overlaps = 0;
    let mut partial_overlaps = 0;

    file_contents.lines().for_each(|line| {
        let (elf1, elf2) = parse_input(line);

        // Total overlap
        if elf1.includes(elf2) || elf2.includes(elf1) {
            total_overlaps += 1;
        }

        // Partial overlap
        if elf1.intersects(elf2) || elf2.intersects(elf1) {
            partial_overlaps += 1;
        }
    });

    println!("Total overlaps: {}", total_overlaps);
    println!("Partial overlaps: {}", partial_overlaps);
}

fn parse_input(line: &str) -> (Range, Range) {
    let get_range = |range_txt: &str| -> Range {
        let mut range = range_txt.split('-').map(|n| n.parse::<u32>().unwrap());
        
        Range::new(range.next().unwrap(), range.next().unwrap())
    };

    let pairs: Vec<Range> = line.split(',').map(get_range).collect();
    (pairs[0], pairs[1])
}