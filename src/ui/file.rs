use tui::{backend::Backend, Frame, layout::{Rect, Layout, Constraint}, widgets::{ListItem, List}, text::{Spans, Span}, style::{Style, Color}};
use crate::git::{GitFile, Git};


pub struct UiFile{}

impl UiFile{

    pub fn new() -> Self {
        Self { 

        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, size: Rect){
        if let Ok(fileString) = Git::get_file() {
            let itemsList: Vec<ListItem> = fileString.iter().map(|file|{
                let mut line: Vec<Spans> = Vec::new();
                match file {
                    GitFile::FileModified(name) => {
                        line.push(
                            Spans::from(
                                Span::styled(name.clone(), Style::default().fg(Color::Blue))
                            )
                        )
                    },
                    GitFile::FileChached(name) => {
                        line.push(
                            Spans::from(
                                Span::styled(name.clone(), Style::default().fg(Color::White))
                            )
                        )
                    },
                    GitFile::FileUntracked(name) => {
                        line.push(
                            Spans::from(
                                Span::styled(name.clone(), Style::default().fg(Color::Green))
                            )
                        )
                    },
                }
                ListItem::new(line).style(Style::default())
            }).collect();
            let items = List::new(itemsList);

            let new_size = Layout::default()
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);
            
            f.render_widget(items, new_size[0]);

        }else{

        }
    }

}