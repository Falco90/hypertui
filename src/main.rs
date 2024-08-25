mod app;
mod hypersync;
mod ui;

use std::{
    error::Error,
    io::{self, Stdout},
};

use app::{App, Erc20Transfer};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use hypersync_client::format::Hex;
use ratatui::prelude::{CrosstermBackend, Terminal};
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    hypersync::query(&mut app).await;

    let mut res = run_app(&mut terminal, &mut app);

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| render_ui(frame, app))?;
    }
}
