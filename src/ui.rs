use ethers::core::utils::format_ether;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{self, Line, Span, Text},
    widgets::{
        Block, Borders, Cell, HighlightSpacing, List, ListItem, Padding, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, Table, Tabs,
    },
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    let centered_rect = centered_rect(95, 95, frame.area());
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().green());
    frame.render_widget(main_block, centered_rect);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(4),
        ])
        .split(centered_rect);

    match app.current_screen {
        CurrentScreen::Startup => render_startup_screen(frame, app, centered_rect),
        CurrentScreen::Main => {
            render_title(frame, app, chunks[0]);
            render_main_screen(frame, app, chunks[1]);
            render_footer(frame, app, chunks[2]);
        }
        CurrentScreen::QueryBuilder => {
            render_title(frame, app, chunks[0]);
            render_query_screen(frame, app, chunks[1]);
            render_footer(frame, app, chunks[2]);
        }
        CurrentScreen::Exiting => {
            render_title(frame, app, chunks[0]);
        }
        CurrentScreen::Loading => {
            render_title(frame, app, chunks[0]);
            render_loading_screen(frame, app, chunks[1]);
        }
    }
}

fn render_main_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(1)])
        .split(area);

    render_tabs(frame, app, chunks[0]);

    match app.transaction_tabs.index {
        0 => render_regular_tab(frame, app, chunks[1]),
        1 => render_erc20_tab(frame, app, chunks[1]),
        2 => render_erc721_tab(frame, app, chunks[1]),
        _ => {}
    }
}

fn render_title(frame: &mut Frame, app: &mut App, area: Rect) {
    let title_block = Block::default().style(Style::default());
    let mut content = "";

    match app.current_screen {
        CurrentScreen::QueryBuilder => content = "\n:: Query Builder ::",
        CurrentScreen::Main => content = "\n:: Query Results ::",
        _ => {}
    }

    let title = Paragraph::new(Text::styled(content, Style::default().fg(Color::Green)))
        .block(title_block)
        .alignment(Alignment::Center);

    frame.render_widget(title, area);
}

fn render_loading_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Loading...",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, area);
}

fn render_startup_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        ".##..##..##..##..#####...######..#####...######..##..##..######.
.##..##...####...##..##..##......##..##....##....##..##....##...
.######....##....#####...####....#####.....##....##..##....##...
.##..##....##....##......##......##..##....##....##..##....##...
.##..##....##....##......######..##..##....##.....####...######.
................................................................",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    let instructions_block = Block::default().style(Style::default());

    let instructions = Paragraph::new(Text::styled(
        "Press 'c' to start a new query\n\nPress 'q' to quit",
        Style::default().fg(Color::Yellow),
    ))
    .block(instructions_block)
    .alignment(Alignment::Center);

    frame.render_widget(title, chunks[1]);
    frame.render_widget(instructions, chunks[2]);
}

fn render_query_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let list_items = vec![
        ListItem::new(Line::from(Span::styled(
            format!("Wallet Address:   {}", app.query.address),
            Style::default().fg(Color::Yellow),
        ))),
        ListItem::new(Line::from(Span::styled(
            format!("Chain:   {}", app.query.chain),
            Style::default().fg(Color::Yellow),
        ))),
    ];

    let list = List::new(list_items)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, area, &mut app.query_state);
}

fn render_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    let tabs = app
        .transaction_tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))).bold())
        .collect::<Tabs>()
        .block(Block::default().style(Style::new().green()).padding(Padding::horizontal(2)))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.transaction_tabs.index);

    frame.render_widget(tabs, area);
}

fn render_regular_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(2)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let right_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);
    let selected_style = Style::default().fg(Color::DarkGray).bg(Color::Yellow);

    let header = ["Hash", "From", "To", "Value"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app.regular_transfers.iter().enumerate().map(|(i, data)| {
        let item = [
            &data.hash,
            &data.from,
            &data.to,
            &format_ether(data.value)[..5],
        ];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
            .height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(200),
            Constraint::Length(40),
            Constraint::Length(40),
            Constraint::Length(40),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .border_style(Style::new().green())
            .padding(Padding::horizontal(2)),
    )
    .highlight_style(selected_style)
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(table, chunks[0], &mut app.table_state);

    render_scrollbar(frame, app, chunks[0]);
    render_tansaction_details(frame, app, right_panel[0]);
}

fn render_scrollbar(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 3,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_erc20_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

    let header = ["Hash", "From", "To", "Value"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app.erc20_transfers.iter().enumerate().map(|(i, data)| {
        let item = [&data.hash, &data.from, &data.to, &data.amount];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
            .height(1)
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(200),
            Constraint::Length(200),
            Constraint::Length(200),
            Constraint::Length(200),
        ],
    )
    .header(header)
    .block(Block::bordered())
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_widget(table, area);
}

fn render_erc721_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

    let header = ["Hash", "Contract", "From", "To", "Token Id"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app.erc721_transfers.iter().enumerate().map(|(i, data)| {
        let item = [
            &data.hash,
            &data.contract,
            &data.from,
            &data.to,
            &data.token_id,
        ];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
            .height(1)
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(200),
            Constraint::Length(200),
            Constraint::Length(200),
            Constraint::Length(200),
            Constraint::Length(200),
        ],
    )
    .header(header)
    .block(Block::bordered())
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_widget(table, area);
}

fn render_tansaction_details(frame: &mut Frame, app: &mut App, area: Rect) {
    if let Some(index) = app.table_state.selected() {
        match app.transaction_tabs.index {
            0 => {
                let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

                let header = ["Transaction Details"]
                    .into_iter()
                    .map(Cell::from)
                    .collect::<Row>()
                    .style(header_style)
                    .height(2);

                let selected_transaction = &app.regular_transfers[index];
                let fields = [
                    ("Hash:   ", selected_transaction.hash.as_str()),
                    ("Block:  ", selected_transaction.block.as_str()),
                    ("From:   ", selected_transaction.from.as_str()),
                    ("To:     ", selected_transaction.to.as_str()),
                    (
                        "Value:   \u{27E0}",
                        &format_ether(selected_transaction.value)[..5],
                    ),
                ];
                let rows = fields.iter().enumerate().map(|(i, data)| {
                    let item = [format!("{} {}", data.0, data.1)];
                    item.into_iter()
                        .map(|content| Cell::from(Text::from(content)))
                        .collect::<Row>()
                        .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
                        .height(1)
                });
                let table = Table::new(rows, [Constraint::Percentage(100)])
                    .header(header)
                    .block(
                        Block::bordered()
                            .border_style(Style::new().green())
                            .padding(Padding::horizontal(2)),
                    );

                frame.render_widget(table, area);
            }
            1 => {}
            2 => {}
            _ => {}
        }
    }
}

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    let instructions_block = Block::default().padding(Padding::vertical(1));
    let mut content = "";

    match app.current_screen {
        CurrentScreen::Main => {
            content = "\nUp: \u{21D1} | Down: \u{21D3} | Next Tab: TAB | Quit: 'q'"
        }
        CurrentScreen::QueryBuilder => {
            content = "\nEdit Mode: 'e' | Up: \u{21D1} | Down: \u{21D3} | Confirm: ENTER"
        }
        _ => {}
    }

    let instructions = Paragraph::new(Text::styled(content, Style::default().fg(Color::Green)))
        .block(instructions_block)
        .alignment(Alignment::Center);

    frame.render_widget(instructions, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
