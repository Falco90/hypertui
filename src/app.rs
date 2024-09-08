use ratatui::widgets::{ListState, ScrollbarState, TableState};
use serde::Serialize;

const LINE_HEIGHT: usize = 1;

pub enum Chain {
    Mainnet(String),
    Optimism(String),
    Arbitrum(String),
}
pub struct WalletQuery {
    pub address: String,
    pub chain: Chain,
    pub regular_transfers: bool,
    pub erc20_transfers: bool,
    pub erc721_transfers: bool,
    pub start_block: String,
}

impl WalletQuery {
    fn new() -> Self {
        WalletQuery {
            address: String::new(),
            chain: Chain::Mainnet("https://eth.hypersync.xyz".to_string()),
            regular_transfers: true,
            erc20_transfers: true,
            erc721_transfers: false,
            start_block: String::from("1"),
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

    pub fn selected(&self) -> Option<TransactionTab> {
        match self.titles[self.index] {
            "Regular Transfers" => Some(TransactionTab::Regular),
            "ERC20 Transfers" => Some(TransactionTab::ERC20),
            "ERC721 Transfers" => Some(TransactionTab::ERC721),
            _ => None,
        }
    }
}

pub enum TransactionTab {
    Regular,
    ERC20,
    ERC721,
}
pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub is_exiting: bool,
    pub is_saving_json: bool,
    pub currently_editing: bool,
    pub query: WalletQuery,
    pub transaction_tabs: TabsState<'a>,
    pub table_states: TableStates,
    pub query_state: ListState,
    pub scrollbar_states: ScrollbarStates,
    pub transfers: Transfers,
}

#[derive(Serialize)]
pub struct RegularTransfer {
    pub hash: String,
    pub block_hash: String,
    pub block: String,
    pub nonce: String,
    pub to: String,
    pub from: String,
    pub value: String,
    pub gas_used: String,
}

#[derive(Serialize)]
pub struct Transfers {
    pub regular_transfers: Vec<RegularTransfer>,
    pub erc20_transfers: Vec<Erc20Transfer>,
    pub erc721_transfers: Vec<Erc721Transfer>,
}

impl Transfers {
    pub fn new() -> Self {
        Transfers {
            regular_transfers: Vec::new(),
            erc20_transfers: Vec::new(),
            erc721_transfers: Vec::new(),
        }
    }
}

pub struct TableStates {
    pub regular_table: TableState,
    pub erc20_table: TableState,
    pub erc721_table: TableState,
}

impl TableStates {
    pub fn new() -> Self {
        TableStates {
            regular_table: TableState::default().with_selected(0),
            erc20_table: TableState::default().with_selected(0),
            erc721_table: TableState::default().with_selected(0),
        }
    }
}

pub struct ScrollbarStates {
    pub regular_scrollbar: ScrollbarState,
    pub erc20_scrollbar: ScrollbarState,
    pub erc721_scrollbar: ScrollbarState,
}

impl ScrollbarStates {
    pub fn new() -> Self {
        ScrollbarStates {
            regular_scrollbar: ScrollbarState::new(0),
            erc20_scrollbar: ScrollbarState::new(0),
            erc721_scrollbar: ScrollbarState::new(0),
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Startup,
            is_exiting: false,
            is_saving_json: false,
            currently_editing: false,
            transaction_tabs: TabsState::new(vec![
                "Regular Transfers",
                "ERC20 Transfers",
                "ERC721 Transfers",
            ]),
            table_states: TableStates::new(),
            scrollbar_states: ScrollbarStates::new(),
            query: WalletQuery::new(),
            query_state: ListState::default().with_selected(Some(0)),
            transfers: Transfers::new(),
        }
    }

    pub fn set_scrollbar_states(&mut self) {
        if !&self.transfers.regular_transfers.is_empty() {
            self.scrollbar_states.regular_scrollbar =
                ScrollbarState::new(&self.transfers.regular_transfers.len() - 1);
        }
        if !&self.transfers.erc20_transfers.is_empty() {
            self.scrollbar_states.erc20_scrollbar =
                ScrollbarState::new(&self.transfers.erc20_transfers.len() - 1);
        }
        if !&self.transfers.erc721_transfers.is_empty() {
            self.scrollbar_states.erc721_scrollbar =
                ScrollbarState::new(&self.transfers.erc721_transfers.len() - 1);
        }
    }

    pub fn next_table_row(&mut self) {
        if let Some(transaction_tab) = self.transaction_tabs.selected() {
            match transaction_tab {
                TransactionTab::Regular => {
                    if !self.transfers.regular_transfers.is_empty() {
                        let i = match self.table_states.regular_table.selected() {
                            Some(i) => {
                                if i >= self.transfers.regular_transfers.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.table_states.regular_table.select(Some(i));
                        self.scrollbar_states.regular_scrollbar = self
                            .scrollbar_states
                            .regular_scrollbar
                            .position(i * LINE_HEIGHT);
                    }
                }
                TransactionTab::ERC20 => {
                    if !self.transfers.erc20_transfers.is_empty() {
                        let i = match self.table_states.erc20_table.selected() {
                            Some(i) => {
                                if i >= self.transfers.erc20_transfers.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.table_states.erc20_table.select(Some(i));
                        self.scrollbar_states.erc20_scrollbar = self
                            .scrollbar_states
                            .erc20_scrollbar
                            .position(i * LINE_HEIGHT);
                    }
                }
                TransactionTab::ERC721 => {
                    if !self.transfers.erc721_transfers.is_empty() {
                        let i = match self.table_states.erc721_table.selected() {
                            Some(i) => {
                                if i >= self.transfers.erc721_transfers.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.table_states.erc721_table.select(Some(i));
                        self.scrollbar_states.erc721_scrollbar = self
                            .scrollbar_states
                            .erc721_scrollbar
                            .position(i * LINE_HEIGHT);
                    }
                }
            }
        }
    }

    pub fn previous_table_row(&mut self) {
        if let Some(transaction_tab) = self.transaction_tabs.selected() {
            match transaction_tab {
                TransactionTab::Regular => {
                    if !self.transfers.regular_transfers.is_empty() {
                        let i = match self.table_states.regular_table.selected() {
                            Some(i) => {
                                if i == 0 {
                                    self.transfers.regular_transfers.len() - 1 * LINE_HEIGHT
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        self.table_states.regular_table.select(Some(i));
                        self.scrollbar_states.regular_scrollbar = self
                            .scrollbar_states
                            .regular_scrollbar
                            .position(i * LINE_HEIGHT);
                    }
                }
                TransactionTab::ERC20 => {
                    if !self.transfers.erc20_transfers.is_empty() {
                        let i = match self.table_states.erc20_table.selected() {
                            Some(i) => {
                                if i == 0 {
                                    self.transfers.erc20_transfers.len() - 1 * LINE_HEIGHT
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        self.table_states.erc20_table.select(Some(i));
                        self.scrollbar_states.erc20_scrollbar = self
                            .scrollbar_states
                            .erc20_scrollbar
                            .position(i * LINE_HEIGHT);
                    }
                }
                TransactionTab::ERC721 => {
                    if !self.transfers.erc721_transfers.is_empty() {
                        let i = match self.table_states.erc721_table.selected() {
                            Some(i) => {
                                if i == 0 {
                                    self.transfers.erc721_transfers.len() - 1 * LINE_HEIGHT
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        self.table_states.erc721_table.select(Some(i));
                        self.scrollbar_states.erc721_scrollbar = self
                            .scrollbar_states
                            .erc721_scrollbar
                            .position(i * LINE_HEIGHT);
                    }
                }
            }
        }
    }
}
