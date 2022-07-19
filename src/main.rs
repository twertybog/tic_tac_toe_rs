mod cli;
mod gui;
use cli::cli;
use gui::gui;

fn main() {
    let arguments: String = std::env::args().collect();
    if arguments.contains("--cli"){
        cli();
    }
    else{
        gui();
    }
}
