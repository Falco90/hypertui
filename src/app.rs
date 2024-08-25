pub struct Erc20Transfer {
    pub block: String,
    pub address: String,
    pub to: String,
    pub from: String,
    pub amount: String
}

pub enum CurrentScreen {
    Startup,
    QueryBuilder,
    Main,
    Exiting,
    Loading
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
    pub widget_index: u8,
    pub regular_transfers: Vec<RegularTransfer>,
    pub erc20_transfers: Vec<Erc20Transfer>
}

pub struct RegularTransfer {
    pub block: String,
    pub to: String,
    pub from: String,
    pub value: String,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Startup,
            tabs: TabsState::new(vec!["regular transfers", "erc20 transfers", "erc721 transfers"]),
            widget_index: 0,
            regular_transfers: Vec::new(),
            erc20_transfers: Vec::new()
        }
    }
}