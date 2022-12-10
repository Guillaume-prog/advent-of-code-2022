use super::{File, Folder, FolderPointer};
use std::{cell::RefCell, rc::Rc};

pub struct FileSystem {
    root_folder: FolderPointer,
    current_folder: FolderPointer
}

impl FileSystem {

    pub fn new() -> Self {
        let root_folder: FolderPointer = Rc::new(RefCell::new(Folder::new("/", None)));

        Self {
            root_folder: root_folder.clone(),
            current_folder: root_folder.clone()
        }
    }

    pub fn get_root(&self) -> FolderPointer {
        self.root_folder.clone()
    }

    pub fn get_size(&self) -> usize {
        self.root_folder.borrow_mut().get_size()
    }

    pub fn generate(&mut self, command_list: String) {
        for command in command_list.lines() {
            let parts: Vec<&str> = command.split(" ").collect();
    
            match &command[0..4] {
                "$ ls" => (),
                "$ cd" => self.handle_cd(parts),
                "dir " => self.handle_new_dir(parts),
                _      => self.handle_new_file(parts)
            };
        }
    }



    fn handle_cd(&mut self, parts: Vec<&str>) {
        let next_folder = parts[2];
        self.current_folder = match next_folder {
            ".." => self.current_folder.borrow_mut().get_parent().unwrap().clone(),
            "/"  => self.root_folder.clone(),
            _    => self.current_folder.borrow_mut().find_child(next_folder).unwrap().clone()
        };
    }

    fn handle_new_dir(&mut self, parts: Vec<&str>) {
        let folder_name = parts[1];

        let folder = Folder::new(folder_name, Some(self.current_folder.clone()));
        self.current_folder.borrow_mut().add_folder(folder);
    }

    fn handle_new_file(&mut self, parts: Vec<&str>) {
        let name = parts[1];
        let size: usize = parts[0].parse().unwrap();

        self.current_folder.borrow_mut().add_file(File::new(name, size));
    }

}