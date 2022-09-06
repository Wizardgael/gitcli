use tui::widgets::ListState;

use crate::git::GitFile;

pub struct StatefulList<T>{
    state: ListState,
    items: Vec<T>
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        Self { 
            state: ListState::default(),
            items
        }
    }
}

pub struct App{
    files: StatefulList<GitFile>
}

impl App {

    pub fn new() -> Self {
        Self { files: StatefulList::with_items(vec![]) }
    }

}