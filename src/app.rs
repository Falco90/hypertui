use ratatui::widgets::{ScrollbarState, TableState};

pub struct Erc20Transfer {
    pub hash: String,
    pub block: String,
    pub contract: String,
    pub to: String,
    pub from: String,
    pub amount: String,
}

pub struct Erc721Transfer {
    pub hash: String,
    pub block: String,
    pub contract: String,
    pub to: String,
    pub from: String,
    pub token_id: String,
}

pub enum CurrentScreen {
    Startup,
    QueryBuilder,
    Main,
    Exiting,
    Loading,
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub tabs: TabsState<'a>,
    pub table_state: TableState,
    pub scroll_state: ScrollbarState,
    pub regular_transfers: Vec<RegularTransfer>,
    pub erc20_transfers: Vec<Erc20Transfer>,
    pub erc721_transfers: Vec<Erc721Transfer>,
}

pub struct RegularTransfer {
    pub hash: String,
    pub block: String,
    pub to: String,
    pub from: String,
    pub value: String,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Startup,
            tabs: TabsState::new(vec![
                "regular transfers",
                "erc20 transfers",
                "erc721 transfers",
            ]),
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(0),
            regular_transfers: Vec::new(),
            erc20_transfers: Vec::new(),
            erc721_transfers: Vec::new(),
        }
    }

    pub fn set_scroll_state(&mut self) {
        self.scroll_state = ScrollbarState::new(&self.regular_transfers.len() - 1);
    }

    pub fn next_table_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.regular_transfers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn previous_table_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.regular_transfers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }
}
