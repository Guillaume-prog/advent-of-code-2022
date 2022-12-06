use std::{fs::File, io::Read};

fn main() {
    let mut signal = String::new();

    // Read the challenge input
    let mut file = File::open("input.txt").expect("Failed to open input file");
    file.read_to_string(&mut signal).expect("Failed to read input file");

    let start_of_packet = search_for_uniq_sequence(&signal, 4).unwrap();
    let start_of_message = search_for_uniq_sequence(&signal, 14).unwrap();
    
    println!("Packet starts at index {}", start_of_packet);
    println!("Message starts at index {}", start_of_message);
}

fn search_for_uniq_sequence(signal: &String, length: usize) -> Option<usize> {
    for i in 0..(signal.len() - length) {
        // Fetch each chunk
        let chunk = signal.get(i..i+length).unwrap();
        let mut uniq_chunk: Vec<char> = Vec::new();

        // Convert chunk into set (only one occurence of each char appears)
        for c in chunk.chars() {
            if !uniq_chunk.contains(&c) {
                uniq_chunk.push(c);
            }
        }

        // If the set has the same length as the chunk, all the chars were unique, we have our marker
        if chunk.len() == uniq_chunk.len() {
            // println!("[{}] {} - {:?}", i, chunk, uniq_chunk);
            return Some(i+length)
        }
    }

    return None;
}