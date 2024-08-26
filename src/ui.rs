use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{self, Line, Span, Text},
    widgets::{Block, Borders, Cell, HighlightSpacing, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::app::App;

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_title(frame, app, chunks[0]);
    render_tabs(frame, app, chunks[1]);

    match app.tabs.index {
        0 => render_regular_tab(frame, app, chunks[2]),
        1 => render_erc20_tab(frame, app, chunks[2]),
        // 2 => render_erc721_tab(frame, app, chunks[1]),
        _ => {}
    }
}

fn render_title(frame: &mut Frame, app: &mut App, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Hypertui Dashboard",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, area);
}

fn render_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered())
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    frame.render_widget(tabs, area);
}

fn render_regular_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

    let header = ["Block", "From", "To", "Value"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app.regular_transfers.iter().enumerate().map(|(i, data)| {
        let item = [&data.block, &data.from, &data.to, &data.value];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
            .height(1)
    });
    let table = Table::new(rows, [
        Constraint::Length(200),
        Constraint::Length(40),
        Constraint::Length(40),
        Constraint::Length(40),
    ])
        .header(header)
        .block(Block::bordered())
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_widget(table, area);
}

fn render_erc20_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

    let header = ["Block", "Contract", "From", "To", "Value"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app.erc20_transfers.iter().enumerate().map(|(i, data)| {
        let item = [&data.block, &data.contract, &data.from, &data.to, &data.amount];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
            .height(1)
    });
    let table = Table::new(rows, [
        Constraint::Length(200),
        Constraint::Length(200),
        Constraint::Length(200),
        Constraint::Length(200),
    ])
        .header(header)
        .block(Block::bordered())
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_widget(table, area);
}

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {}

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
