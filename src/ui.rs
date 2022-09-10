
mod file;

use std::{error::{Error}, io, time::{Duration, Instant}};

use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, 
    execute, 
    event::{EnableMouseCapture, Event, KeyCode, DisableMouseCapture, self}
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

use crate::app::App;

use self::file::UiFile;

pub struct UI{
   file_component: UiFile 
}


impl UI{

    pub fn new() -> Self {
        Self {  
            file_component: UiFile::new()
        }
    }

    pub fn init(&self, app: &mut App) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal, app);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    fn run_app<B: Backend>(&self, terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
        // let tick_rate = Duration::from_millis(250);
        // let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| self.ui(f, app))?;
            //let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));
            //if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.files.unselect(),
                        KeyCode::Down => app.files.next(),
                        KeyCode::Up => app.files.previous(),
                        _ => {},
                    }
                }
            //}
            // if last_tick.elapsed() >= tick_rate {
            //     last_tick = Instant::now();
            // }
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, app: &mut App){

        let block = Block::default()
        .borders(Borders::ALL)
        .title(" GitCli ")
        .title_alignment(tui::layout::Alignment::Left)
        .border_type(tui::widgets::BorderType::Rounded);
        
        f.render_widget(block, f.size());

        let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(75), 
            Constraint::Percentage(25)
        ].as_ref())
        .split(f.size());


        let files = Block::default()
        .borders(Borders::ALL)
        .title(" Files ")
        .title_alignment(tui::layout::Alignment::Left)
        .border_type(tui::widgets::BorderType::Rounded);

        f.render_widget(files, chunks[1]);

        self.file_component.render(f, chunks[1], app);

        let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(chunks[0]);

        let commands = Block::default()
        .borders(Borders::ALL)
        .title(" Commands ")
        .title_alignment(tui::layout::Alignment::Left)
        .border_type(tui::widgets::BorderType::Rounded);
        f.render_widget(commands, main_chunks[0]);

        let output = Block::default()
        .borders(Borders::ALL)
        .title(" Output ")
        .title_alignment(tui::layout::Alignment::Left)
        .border_type(tui::widgets::BorderType::Rounded);
        f.render_widget(output, main_chunks[1]);
    }
    
}