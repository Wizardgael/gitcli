use std::{error::{Error}, io};

use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, 
    execute, 
    event::{EnableMouseCapture, Event, KeyCode, DisableMouseCapture}
};
use tui::{
    backend::{
        CrosstermBackend, 
        Backend
    }, 
    Terminal, 
    Frame, 
    widgets::{
        Block, 
        Borders
    }, 
    layout::{Layout, Direction, Constraint}
};

pub struct UI{
    
}

impl UI{

    pub fn new() -> Self {
        Self {  

        }
    }

    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    fn run_app<B: Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(UI::ui)?;

            if let Event::Key(key) = crossterm::event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(())
                }
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>){
        let size = f.size();

        let block = Block::default()
        .borders(Borders::ALL)
        .title(" GitCli ")
        .title_alignment(tui::layout::Alignment::Left)
        .border_type(tui::widgets::BorderType::Rounded);
        
        f.render_widget(block, size);

        let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Percentage(50)
        ].as_ref())
        .split(f.size());


        let files = Block::default()
        .borders(Borders::ALL)
        .title(" Files ")
        .title_alignment(tui::layout::Alignment::Left)
        .border_type(tui::widgets::BorderType::Rounded);

        f.render_widget(files, chunks[1]);

    }
    
}