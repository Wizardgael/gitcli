use app::App;
use ui::UI;
use std::error::Error; 

mod git;
mod ui;
mod app;

fn main() -> Result<(), Box<dyn Error>>{
    let ui = UI::new();
    let mut app = App::new();
    ui.init(&mut app)?;
    Ok(())

}
