mod app;
mod hypersync;
mod ui;

use app::{App, Chain, CurrentScreen};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use serde_json;
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufWriter, Stdout, Write},
};
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let _res =run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app<'a>(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App<'a>,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| render_ui(frame, app))?;

        if let CurrentScreen::Loading = &app.current_screen {
            hypersync::query(app).await;
            app.set_scrollbar_states();
            app.current_screen = CurrentScreen::Main;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            if !app.currently_editing && !app.is_exiting {
                match key.code {
                    KeyCode::Char('q') => app.is_exiting = true,
                    _ => {}
                }
            }

            if app.is_exiting {
                match key.code {
                    KeyCode::Char('y') => return Ok(true),
                    KeyCode::Char('n') => app.is_exiting = false,
                    _ => {}
                }
            }

            if app.is_saving_json {
                match key.code {
                    KeyCode::Char('y') => {
                        write_to_json(app)?;
                        app.is_saving_json = false;
                    }
                    KeyCode::Char('n') => {
                        app.is_saving_json = false;
                    }
                    _ => {}
                }
            }

            match app.current_screen {
                CurrentScreen::Startup => match key.code {
                    KeyCode::Char('c') => {
                        app.current_screen = CurrentScreen::QueryBuilder;
                    }
                    _ => {}
                },
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('c') => {
                        app.current_screen = CurrentScreen::QueryBuilder;
                    }
                    KeyCode::Char('j') => {
                        app.is_saving_json = true;
                    }
                    KeyCode::Tab => {
                        app.transaction_tabs.next();
                    }
                    KeyCode::Up => {
                        app.previous_table_row();
                    }
                    KeyCode::Down => {
                        app.next_table_row();
                    }
                    _ => {}
                },
                CurrentScreen::QueryBuilder => {
                    if !app.currently_editing {
                        match key.code {
                            KeyCode::Char('y') => {
                                app.current_screen = CurrentScreen::Loading;
                            }
                            KeyCode::Char('e') => {
                                app.currently_editing = true;
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Esc => {
                                app.currently_editing = false;
                            }
                            KeyCode::Up => {
                                app.query_state.select_previous();
                            }
                            KeyCode::Down => {
                                app.query_state.select_next();
                            }
                            KeyCode::Char(value) => {
                                if app.currently_editing {
                                    match app.query_state.selected().unwrap() {
                                        0 => app.query.address.push(value),
                                        5 => app.query.start_block.push(value),
                                        _ => {}
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                if app.currently_editing {
                                    match app.query_state.selected().unwrap() {
                                        0 => {
                                            app.query.address.pop();
                                        }
                                        5 => {
                                            app.query.start_block.pop();
                                        }
                                        _ => {}
                                    };
                                }
                            }
                            KeyCode::Enter => match app.query_state.selected().unwrap() {
                                0 => {
                                    app.query_state.select(Some(1));
                                }
                                1 => {
                                    app.query.regular_transfers = !app.query.regular_transfers;
                                }
                                2 => {
                                    app.query.erc20_transfers = !app.query.erc20_transfers;
                                }
                                3 => {
                                    app.query.erc721_transfers = !app.query.erc721_transfers;
                                }
                                4 => {
                                    app.query.chain = match app.query.chain {
                                        Chain::Mainnet(_) => Chain::Optimism(
                                            "https://optimism.hypersync.xyz".to_string(),
                                        ),
                                        Chain::Optimism(_) => Chain::Arbitrum(
                                            "https://arbitrum.hypersync.xyz".to_string(),
                                        ),
                                        Chain::Arbitrum(_) => {
                                            Chain::Mainnet("https://eth.hypersync.xyz".to_string())
                                        }
                                    };
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                CurrentScreen::Loading => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Startup;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn write_to_json(app: &App) -> io::Result<()> {
    let file = File::create(format!(
        "outputs/{}-{}.json",
        app.query.address,
        match app.query.chain {
            Chain::Mainnet(_) => "mainnet",
            Chain::Optimism(_) => "optimism",
            Chain::Arbitrum(_) => "arbitrum",
        }
    ))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &app.transfers)?;
    writer.flush()?;
    Ok(())
}
