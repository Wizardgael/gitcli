use tui::widgets::ListState;

use crate::git::GitFile;

pub struct StatefulList<T>{
    pub state: ListState,
    pub items: Vec<T>
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        Self { 
            state: ListState::default(),
            items
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                }else {
                    i + 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                }else{
                    i - 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn unselect(&mut self) {
        self.state.select(None)
    }
    
}

pub struct App{
    pub files: StatefulList<GitFile>
}

impl App {

    pub fn new() -> Self {
        Self { files: StatefulList::with_items(vec![]) }
    }

    pub fn update_list(&mut self, list: Vec<GitFile>){
        let selected = match self.files.state.selected() {
            Some(i) => {
                if i >= list.len() - 1 {
                    Some(list.len() - 1)
                }else{
                    Some(i)
                }
            },
            None => None
        }; 
        self.files = StatefulList::with_items(list);
        self.files.state.select(selected);
    }
}