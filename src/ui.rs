use ethers::core::utils::format_ether;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{self, Line, Span, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, Cell, HighlightSpacing, List, ListItem, Padding,
        Paragraph, Row, Scrollbar, ScrollbarOrientation, Table, Tabs,
    },
    Frame,
};

use crate::app::{App, Chain, CurrentScreen};

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
        CurrentScreen::Loading => content = "\n:: Processing Query ::",
        CurrentScreen::Main => content = "\n:: Query Results ::",
        _ => {}
    }

    let title = Paragraph::new(Text::styled(content, Style::default().fg(Color::Green)))
        .block(title_block)
        .alignment(Alignment::Center);

    frame.render_widget(title, area);
}

fn render_loading_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let pop_up = centered_rect(60, 60, area);
    let title_block = Block::default().style(Style::default().green());

    let text = Paragraph::new(Text::styled(
        format!("HyperSync is fetching {} {} {} \nfrom block {} on \n{}...", match app.query.regular_transfers {
            true => "\nRegular Transfers",
            false => ""
        },
        match app.query.erc20_transfers {
            true => "\nERC20 Transfers",
            false => ""
        },
        match app.query.erc721_transfers {
            true => "\nERC721 Transfers",
            false => ""
        },
        app.query.start_block,
        match app.query.chain {
            Chain::Mainnet(_) => "Mainnet",
            Chain::Optimism(_) => "Mainnet",
            Chain::Arbitrum(_) => "Mainnet"
        }
    ),
        Style::default().fg(Color::Yellow),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    frame.render_widget(text, pop_up);
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
    let pop_up = centered_rect(60, 60, area);

    let list_items = vec![
        ListItem::new(Line::from(Span::styled(
            format!("Wallet Address:            {}", app.query.address),
            Style::default().fg(Color::Yellow),
        ))),
        ListItem::new(Line::from(Span::styled(
            format!(
                "Regular transfers:         {}",
                match app.query.regular_transfers {
                    true => "Yes",
                    false => "No",
                }
            ),
            Style::default().fg(Color::Yellow),
        ))),
        ListItem::new(Line::from(Span::styled(
            format!(
                "ERC20 transfers:           {}",
                match app.query.erc20_transfers {
                    true => "Yes",
                    false => "No",
                }
            ),
            Style::default().fg(Color::Yellow),
        ))),
        ListItem::new(Line::from(Span::styled(
            format!(
                "ERC721 transfers:          {}",
                match app.query.erc721_transfers {
                    true => "Yes",
                    false => "No",
                }
            ),
            Style::default().fg(Color::Yellow),
        ))),
        ListItem::new(Line::from(Span::styled(
            format!(
                "Chain:                     {}",
                match app.query.chain {
                    Chain::Mainnet(_) => "Mainnet",
                    Chain::Optimism(_) => "Optimism",
                    Chain::Arbitrum(_) => "Arbitrum",
                }
            ),
            Style::default().fg(Color::Yellow),
        ))),
        ListItem::new(Line::from(Span::styled(
            format!("From block:                {}", app.query.start_block),
            Style::default().fg(Color::Yellow)
        )))
    ];

    let list = List::new(list_items)
        .highlight_symbol("> ")
        .highlight_spacing(HighlightSpacing::Always)
        .block(
            Block::default()
                .green()
                .borders(Borders::ALL)
                .padding(Padding::uniform(2)),
        );

    frame.render_stateful_widget(list, pop_up, &mut app.query_state);
}

fn render_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    let tabs = app
        .transaction_tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))).bold())
        .collect::<Tabs>()
        .block(
            Block::default()
                .style(Style::new().green())
                .padding(Padding::horizontal(2)),
        )
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

    let bottom_right_panel = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(right_panel[1]);

    let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);
    let selected_style = Style::default().fg(Color::DarkGray).bg(Color::Yellow);

    let header = ["Hash", "From", "To", "Value"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app
        .transfers
        .regular_transfers
        .iter()
        .enumerate()
        .map(|(i, data)| {
            let item = [&data.hash, &data.from, &data.to, &data.value[..5]];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("{}", truncate(content)))))
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
    frame.render_stateful_widget(table, chunks[0], &mut app.table_states.regular_table);

    render_scrollbar(frame, app, chunks[0]);
    render_tansaction_details(frame, app, right_panel[0]);
    render_bar_chart(frame, app, bottom_right_panel[0])
}

fn render_scrollbar(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .style(Style::new().green())
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 3,
            horizontal: 1,
        }),
        &mut app.scrollbar_states.regular_scrollbar,
    );
}

fn render_erc20_tab(frame: &mut Frame, app: &mut App, area: Rect) {
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
    let rows = app
        .transfers
        .erc20_transfers
        .iter()
        .enumerate()
        .map(|(i, data)| {
            let item = [&data.hash, &data.from, &data.to, &data.amount];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("{}", truncate(content)))))
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
    frame.render_stateful_widget(table, chunks[0], &mut app.table_states.regular_table);

    render_scrollbar(frame, app, chunks[0]);
    render_tansaction_details(frame, app, right_panel[0]);
}

fn render_erc721_tab(frame: &mut Frame, app: &mut App, area: Rect) {
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

    let header = ["Hash", "From", "To", "TokenId"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app
        .transfers
        .erc721_transfers
        .iter()
        .enumerate()
        .map(|(i, data)| {
            let item = [&data.hash, &data.from, &data.to, &data.token_id];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("{}", truncate(content)))))
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
    frame.render_stateful_widget(table, chunks[0], &mut app.table_states.regular_table);

    render_scrollbar(frame, app, chunks[0]);
    render_tansaction_details(frame, app, right_panel[0]);
}

fn render_tansaction_details(frame: &mut Frame, app: &mut App, area: Rect) {
    if let Some(index) = app.table_states.regular_table.selected() {
        match app.transaction_tabs.index {
            0 => {
                let header_style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

                let header = ["Transaction Details"]
                    .into_iter()
                    .map(Cell::from)
                    .collect::<Row>()
                    .style(header_style)
                    .height(2);

                let selected_transaction = &app.transfers.regular_transfers[index];
                let fields = [
                    ("Hash:   ", selected_transaction.hash.as_str()),
                    ("Block:  ", selected_transaction.block.as_str()),
                    ("From:   ", selected_transaction.from.as_str()),
                    ("To:     ", selected_transaction.to.as_str()),
                    ("Value:   \u{27E0}", &selected_transaction.value[..5]),
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

fn render_bar_chart(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut highest_values: Vec<f64> = Vec::new();
    let mut high_values: Vec<f64> = Vec::new();
    let mut medium_values: Vec<f64> = Vec::new();
    let mut low_values: Vec<f64> = Vec::new();
    let mut lowest_values: Vec<f64> = Vec::new();

    for transaction in &app.transfers.regular_transfers {
        let parsed_value = transaction.value.as_str().parse::<f64>().unwrap();
        if parsed_value >= 5.0 {
            highest_values.push(parsed_value);
        } else if parsed_value >= 1.0 {
            high_values.push(parsed_value);
        } else if parsed_value >= 0.5 {
            medium_values.push(parsed_value);
        } else if parsed_value >= 0.1 {
            low_values.push(parsed_value)
        } else {
            lowest_values.push(parsed_value)
        }
    }

    let bars: Vec<Bar> = vec![
        lowest_values.len(),
        low_values.len(),
        medium_values.len(),
        high_values.len(),
        highest_values.len(),
    ]
    .iter()
    .map(|v| *v)
    .enumerate()
    .map(|(i, value)| {
        Bar::default()
            .value(value.try_into().unwrap())
            .label(Line::from(format!(
                "{}",
                match i {
                    0 => "<0.1",
                    1 => "0.1-0.5",
                    2 => "0.5-1",
                    3 => "1-5",
                    4 => ">5",
                    _ => "",
                }
            )))
            .text_value(format!("{value}"))
            .style(Style::new().yellow())
            .value_style(Style::new())
    })
    .collect();
    let title = Line::from("Transaction Values").centered();

    let bar_chart = BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(
            Block::new()
                .title(title)
                .borders(Borders::ALL)
                .padding(Padding::symmetric(1, 0))
                .style(Style::new().green()),
        )
        .bar_width(7);

    frame.render_widget(bar_chart, area)
}

fn render_regular_statistics(frame: &mut Frame, app: &mut App, area: Rect) {}

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

fn truncate(content: &str) -> String {
    if content.len() >= 2 && content[..2] == *"0x" {
        format!(
            "{}...{}",
            &content[..4],
            &content[content.len() - 4..content.len()]
        )
    } else {
        content.to_string()
    }
}
