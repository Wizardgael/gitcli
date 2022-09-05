use ui::UI;
use std::error::Error; 

mod git;
mod ui;

fn main() -> Result<(), Box<dyn Error>>{
    let ui = UI::new();
    ui.init()?;
    Ok(())

}
