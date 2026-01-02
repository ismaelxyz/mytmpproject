use std::io;
use mytmpproject::ui;

fn main() -> io::Result<()> {
    ui::play_game(std::io::stdin().lock(), std::io::stdout())
}
