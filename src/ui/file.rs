use tui::{backend::Backend, Frame, layout::{Rect, Layout, Constraint}, widgets::{ListItem, List}, text::{Spans, Span}, style::{Style, Color, Modifier}};
use crate::{git::{GitFile, Git}, app::App};


pub struct UiFile{}

impl UiFile{

    pub fn new() -> Self {
        Self { 

        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, size: Rect, app: &mut App){
        if let Ok(file_string) = Git::get_file() {
            app.update_list(file_string);
            let items_list: Vec<ListItem> = app.files.items.iter().map(|file| {
                let mut line: Vec<Spans> = Vec::new();
                match file {
                    GitFile::FileModified(name) => {
                        line.push(
                            Spans::from(vec![
                                Span::styled(name.clone(), Style::default().fg(Color::Blue))
                            ])
                        )
                    },
                    GitFile::FileChached(name) => {
                        
                        line.push(
                            Spans::from(vec![
                                Span::styled(name.clone(), Style::default().fg(Color::White))
                            ])
                        )
                    },
                    GitFile::FileUntracked(name) => {
                        line.push(
                            Spans::from(vec![
                                Span::styled(name.clone(), Style::default().fg(Color::Green))
                            ])  
                        )
                    },
                }
                ListItem::new(line).style(Style::default())
            }).collect();

            let items = List::new(items_list)
            .highlight_style(Style::default().bg(Color::LightBlue).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

            let new_size = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);
            
            f.render_stateful_widget(items, new_size[0], &mut app.files.state);

        }else{

        }
    }

}