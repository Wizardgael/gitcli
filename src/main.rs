use git::Git;
use ui::UI;

mod git;
mod ui;

fn main() {
    
    let mut ui = UI::new();
    ui.init();

}
