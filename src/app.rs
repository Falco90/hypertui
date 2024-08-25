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
pub struct App {
    pub current_screen: CurrentScreen,
    pub regular_transfers: Vec<RegularTransfer>,
    pub erc20_transfers: Vec<Erc20Transfer>
}

pub struct RegularTransfer {
    pub block: String,
    pub to: String,
    pub from: String,
    pub value: String
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Startup,
            regular_transfers: Vec::new(),
            erc20_transfers: Vec::new()
        }
    }
}