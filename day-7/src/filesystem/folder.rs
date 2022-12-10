use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::slice::Iter;

use super::file::File;

pub type FolderPointer = Rc<RefCell<Folder>>;

#[derive(Debug)]
pub struct Folder {
    pub name: String,
    parent: Option<FolderPointer>,
    pub subfolders: Vec<FolderPointer>,
    pub files: Vec<File>
}

impl Folder { 

    pub fn new(name: &str, parent: Option<FolderPointer>) -> Self {
        Self { 
            name: name.to_string(),
            parent: parent,
            subfolders: Vec::new(),
            files: Vec::new()
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.subfolders.push(Rc::new(RefCell::new(folder)));
    }

    pub fn get_size(&self) -> usize {
        let subfolders_size: usize = self.subfolders.iter().map(|folder| folder.borrow().get_size()).sum();
        let files_size: usize = self.files.iter().map(|file| file.size).sum();

        subfolders_size + files_size
    }

    pub fn get_subfolders(&self) -> Iter<'_, FolderPointer> {
        self.subfolders.iter()
    }

    pub fn get_parent(&self) -> Option<FolderPointer> {
        return self.parent.as_ref().map(|f| f.clone());
    }

    pub fn find_child(&self, name: &str) -> Option<FolderPointer> {
        let folder = self.subfolders
            .iter()
            .filter(|folder| folder.borrow().name == name.to_string())
            .next();

        folder.map(|f| f.clone())
    }

    fn get_display_str(&self, tab_level: usize) -> String {
        let mut output:Vec<String> = Vec::new();
        let tabs = " ".repeat(tab_level * 2);
        
        output.push(format!("{}- [dir] {}", tabs, self.name));

        for subfolder in &self.subfolders {
            output.push(subfolder.borrow().get_display_str(tab_level+1));
        }

        for file in &self.files {
            output.push(format!("{}  - [fle] {}", tabs, file));
        }

        output.join("\n")
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        

        write!(f, "{}", self.get_display_str(0))
    }
}