use std::sync::Arc;

use hypersync_client::{
    format::Hex, net_types::Query, Client, ClientConfig, Decoder, StreamConfig,
};

use crate::app::{App, Erc20Transfer};
// Convert address (20 bytes) to hash (32 bytes) so it can be used as a topic filter.
// Pads the address with zeroes.
fn address_to_topic(address: &str) -> String {
    format!("0x000000000000000000000000{}", &address[2..])
}

pub async fn query(app: &mut App) {
    let client = Client::new(ClientConfig {
        url: Some("https://eth.hypersync.xyz".parse().unwrap()),
        ..Default::default()
    })
    .unwrap();

    let addresses = vec![
        "0xD1a923D70510814EaE7695A76326201cA06d080F",
        "0xc0A101c4E9Bb4463BD2F5d6833c2276C36914Fb6",
        "0xa0FBaEdC4C110f5A0c5E96c3eeAC9B5635b74CE7",
        "0x32448eb389aBe39b20d5782f04a8d71a2b2e7189",
    ];

    // pad our addresses so we can use them as a topic filter
    let address_topic_filter: Vec<String> = addresses.iter().map(|a| address_to_topic(a)).collect();

    let query: Query = serde_json::from_value(serde_json::json!( {
        // start from block 0 and go to the end of the chain (we don't specify a toBlock).
        "from_block": 0,
        // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
        "logs": [
            {
                // We want All ERC20 transfers coming to any of our addresses
                "topics":[
                    ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
                    [],
                    // in the "to" position here
                    address_topic_filter,
                    [],
                ]
            },
            {
                // We want All ERC20 transfers coming from any of our addresses
                "topics":[
                    ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
                    // in the "from" position here
                    address_topic_filter,
                    [],
                    [],
                ]
            },
        ],
        "transactions": [
            // get all transactions coming from OR going to any of our addresses
            {"from": addresses},
            {"to": addresses}
        ],
        // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
        "field_selection": {
            "log": [
                "block_number",
                "address",
                "data",
                "topic0",
                "topic1",
                "topic2",
                "topic3",
            ],
            "transaction": [
                "block_number",
                "from",
                "to",
                "value",
            ]
        },
    }))
    .unwrap();

    let client = Arc::new(client);

    println!("Fetching data through hypersync...");

    // Stream data
    //
    // This will parallelize internal requests so we don't have to worry about pipelining/parallelizing make request -> handle response -> handle data loop
    let mut receiver = client.stream(query, StreamConfig::default()).await.unwrap();

    let decoder = Decoder::from_signatures(&[
        "Transfer(address indexed from, address indexed to, uint amount)",
    ])
    .unwrap();

    while let Some(res) = receiver.recv().await {
        let res = res.unwrap();

        for batch in res.data.logs {
            for log in batch {
                if let Ok(decoded_log) = decoder.decode_log(&log) {
                    let decoded_log = decoded_log.unwrap();
                    app.erc20transfers.push(Erc20Transfer {
                        block: log.block_number.unwrap().to_string(),
                        from: decoded_log.indexed[0].as_address().unwrap().to_string(),
                        to: decoded_log.indexed[0].as_address().unwrap().to_string(),
                        amount: decoded_log.body[0].as_uint().unwrap().0.to_string()
                    });
                }
            }
        }

        for batch in res.data.transactions {
            for tx in batch {
                app.transactions.push(tx);
            }
        }
    }
}
