use std::sync::Arc;

use hypersync_client::{
    format::Hex, net_types::Query, Client, ClientConfig, Decoder, StreamConfig,
};

use crate::app::{App, Erc20Transfer, RegularTransfer};
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

    let address_topic_filter: Vec<String> = addresses.iter().map(|a| address_to_topic(a)).collect();

    let query: Query = serde_json::from_value(serde_json::json!( {
        "from_block": 0,
        "logs": [
            {
                "topics":[
                    ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
                    [],
                    address_topic_filter,
                    [],
                ]
            },
            {
                "topics":[
                    ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
                    address_topic_filter,
                    [],
                    [],
                ]
            },
        ],
        "transactions": [
            {"from": addresses},
            {"to": addresses}
        ],
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
                    app.erc20_transfers.push(Erc20Transfer {
                        block: log.block_number.unwrap().to_string(),
                        address: log.address.unwrap().encode_hex(),
                        from: decoded_log.indexed[0].as_address().unwrap().to_string(),
                        to: decoded_log.indexed[0].as_address().unwrap().to_string(),
                        amount: decoded_log.body[0].as_uint().unwrap().0.to_string(),
                    });
                }
            }
        }

        for batch in res.data.transactions {
            for tx in batch {
                app.regular_transfers.push(RegularTransfer {
                    block: tx.block_number.unwrap().to_string(),
                    from: tx.from.unwrap().encode_hex(),
                    to: tx.to.unwrap().encode_hex(),
                    value: tx.value.unwrap().encode_hex(),
                });
            }
        }
    }
}
