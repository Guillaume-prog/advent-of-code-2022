mod filesystem;
use filesystem::{ FolderPointer, FileSystem };

use std::{fs::File as F, io::Read};

fn main() {
    let command_list = get_command_list("input.txt");
    
    let mut filesystem = FileSystem::new();
    filesystem.generate(command_list);

    // Get folder sizes
    let mut folders_sizes: Vec<usize> = Vec::new();
    get_folders_sizes(&mut folders_sizes, filesystem.get_root());

    // Get sum of all files under 100_000 bytes
    let total_size: usize = folders_sizes.iter()
        .filter(|size| size < &&100_000)
        .sum();

    // Get the size of the file to be deleted
    let current_free_space: usize = 70_000_000 - filesystem.get_size();

    let size_of_file_to_be_deleted: usize = *folders_sizes.iter()
        .filter(|size| **size + current_free_space >= 30_000_000)
        .min().unwrap();

    // Print results
    println!("Sum of all folders under 100_000: {} bytes", total_size);
    println!("Size of directory to delete: {} bytes", size_of_file_to_be_deleted);
}

fn get_folders_sizes(folder_list: &mut Vec<usize>, current_folder: FolderPointer) {
    folder_list.push(current_folder.borrow_mut().get_size());

    for folder in current_folder.borrow_mut().get_subfolders() {
        get_folders_sizes(folder_list, folder.clone());
    }
}

fn get_command_list(path: &str) -> String {
    let mut command_list = String::new();

    // Read the challenge input
    let mut file = F::open(path).expect("Failed to open input file");
    file.read_to_string(&mut command_list).expect("Failed to read input file");

    command_list
}