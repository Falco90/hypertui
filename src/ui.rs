use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_title(frame, app, chunks[0]);

    match app.tabs.index {
        0 => render_regular_tab(frame, app, chunks[1]),
        1 => render_erc20_tab(frame, app, chunks[1]),
        2 => render_erc721_tab(frame, app, chunks[1]),
        _ => {}
    }
    // render_list(frame, app, chunks[1]);
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

fn render_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut list_items = Vec::<ListItem>::new();

    for erc20transfer in &app.erc20_transfers {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!(
                "Erc20 transfer: block: {}, from: {} to: {} value: {}",
                erc20transfer.block, erc20transfer.from, erc20transfer.to, erc20transfer.amount
            ),
            Style::default().fg(Color::Yellow),
        ))));
    }

    for regular_transfer in &app.regular_transfers {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!(
                "Regular transfer: block: {}, from: {} to: {} value: {}",
                regular_transfer.block,
                regular_transfer.from,
                regular_transfer.to,
                regular_transfer.value
            ),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, area);
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
