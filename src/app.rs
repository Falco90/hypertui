use hypersync_client::{format::Address, simple_types::{Log, Transaction}};

pub struct Erc20Transfer {
    pub block: String,
    pub to: String,
    pub from: String,
    pub amount: String
}
pub struct App {
    pub transactions: Vec<Transaction>,
    pub erc20transfers: Vec<Erc20Transfer>
}

impl App {
    pub fn new() -> Self {
        App {
            transactions: Vec::new(),
            erc20transfers: Vec::new()
        }
    }
}