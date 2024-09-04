use ratatui::widgets::{ListState, ScrollbarState, TableState};
use ethers::core::types::U256;
use serde::Serialize;

const LINE_HEIGHT: usize = 1;

pub struct WalletQuery {
    pub address: String,
    pub chain: String,
    pub regular_transfers: bool,
    pub erc20_transfers: bool,
    pub erc721_transfers: bool,
    pub index: usize,
}

impl WalletQuery {
    fn new() -> Self {
        WalletQuery {
            address: String::new(),
            chain: String::new(),
            regular_transfers: true,
            erc20_transfers: true,
            erc721_transfers: false,
            index: 0,
        }
    }
}

pub struct QueryListState<'b> {
    items: Vec<&'b str>,
    index: usize,
}

impl<'b> QueryListState<'b> {
    pub fn new(items: Vec<&'b str>) -> Self {
        Self { items, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.items.len() - 1;
        }
    }
}

#[derive(Serialize)]
pub struct Erc20Transfer {
    pub hash: String,
    pub block: String,
    pub contract: String,
    pub to: String,
    pub from: String,
    pub amount: String,
}

#[derive(Serialize)]
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
    pub currently_editing: bool,
    pub query: WalletQuery,
    pub transaction_tabs: TabsState<'a>,
    pub table_state: TableState,
    pub query_tabs: TabsState<'a>,
    pub query_state: ListState,
    pub scroll_state: ScrollbarState,
    pub transfers: Transfers
}

#[derive(Serialize)]
pub struct RegularTransfer {
    pub hash: String,
    pub block: String,
    pub to: String,
    pub from: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct Transfers {
    pub regular_transfers: Vec<RegularTransfer>,
    pub erc20_transfers: Vec<Erc20Transfer>,
    pub erc721_transfers: Vec<Erc721Transfer>
}

impl Transfers {
    fn new() -> Self {
        Transfers {
            regular_transfers: Vec::new(),
            erc20_transfers: Vec::new(),
            erc721_transfers: Vec::new()
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Startup,
            currently_editing: false,
            transaction_tabs: TabsState::new(vec![
                "Regular Transfers",
                "ERC20 Transfers",
                "ERC721 Transfers",
            ]),
            query_tabs: TabsState::new(vec!["Address", "Chain", "Types"]),
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(0),
            query: WalletQuery::new(),
            query_state: ListState::default().with_selected(Some(0)),
            transfers: Transfers::new()
        }
    }

    pub fn set_scroll_state(&mut self) {
        self.scroll_state = ScrollbarState::new(&self.transfers.regular_transfers.len() - 1);
    }

    pub fn next_table_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.transfers.regular_transfers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * LINE_HEIGHT);
    }

    pub fn previous_table_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.transfers.regular_transfers.len() - 1 * LINE_HEIGHT
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * LINE_HEIGHT);
    }
}
