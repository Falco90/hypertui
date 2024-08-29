use std::sync::Arc;

use hypersync_client::{
    format::Hex, net_types::Query, Client, ClientConfig, Decoder, StreamConfig,
};
use serde_json::Value;

use crate::app::{App, CurrentScreen, Erc20Transfer, Erc721Transfer, RegularTransfer};
fn address_to_topic(address: &str) -> String {
    format!("0x000000000000000000000000{}", &address[2..])
}

pub async fn query<'a>(app: &mut App<'a>) {
    let client = Client::new(ClientConfig {
        url: Some("https://eth.hypersync.xyz".parse().unwrap()),
        ..Default::default()
    })
    .unwrap();

    let addresses = vec!["0xad4010aC206b14D66999b4BF9b80C6bc97B60b9A"];

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
            "log": Value::Array(vec![
                "transaction_hash".into(),
                "block_number".into(),
                "address".into(),
                "data".into(),
                "topic0".into(),
                "topic1".into(),
                "topic2".into(),
                "topic3".into(),
            ]),
            "transaction": [
                "hash",
                "block_number",
                "from",
                "to",
                "value",
            ]
        },
    }))
    .unwrap();

    let client = Arc::new(client);

    let mut receiver = client.stream(query, StreamConfig::default()).await.unwrap();

    let decoder = Decoder::from_signatures(&[
        "Transfer(address indexed from, address indexed to, uint amount)",
    ])
    .unwrap();

    let erc721_decoder = Decoder::from_signatures(&[
        "Transfer(address indexed from, address indexed to, uint indexed tokenId)",
    ])
    .unwrap();

    while let Some(res) = receiver.recv().await {
        let res = res.unwrap();

        for batch in res.data.logs {
            for log in batch {
                match &log.data {
                    Some(_) => {
                        if let Ok(decoded_log) = decoder.decode_log(&log) {
                            match &decoded_log {
                                Some(_) => {
                                    let decoded_log = decoded_log.unwrap();
                                    app.erc20_transfers.push(Erc20Transfer {
                                        hash: log.transaction_hash.unwrap().encode_hex(),
                                        block: log.block_number.unwrap().to_string(),
                                        contract: log.address.unwrap().encode_hex(),
                                        from: decoded_log.indexed[0]
                                            .as_address()
                                            .unwrap()
                                            .to_string(),
                                        to: decoded_log.indexed[1]
                                            .as_address()
                                            .unwrap()
                                            .to_string(),
                                        amount: decoded_log.body[0]
                                            .as_uint()
                                            .unwrap()
                                            .0
                                            .to_string(),
                                    });
                                }
                                None => {
                                    if let Ok(decoded_log) = erc721_decoder.decode_log(&log) {
                                        let decoded_log = decoded_log.unwrap();
                                        app.erc721_transfers.push(Erc721Transfer {
                                            hash: log.transaction_hash.unwrap().encode_hex(),
                                            block: log.block_number.unwrap().to_string(),
                                            contract: log.address.unwrap().encode_hex(),
                                            from: decoded_log.indexed[0]
                                                .as_address()
                                                .unwrap()
                                                .to_string(),
                                            to: decoded_log.indexed[1]
                                                .as_address()
                                                .unwrap()
                                                .to_string(),
                                            token_id: decoded_log.indexed[2]
                                                .as_uint()
                                                .unwrap()
                                                .0
                                                .to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        for batch in res.data.transactions {
            for tx in batch {
                let regular_transfer = RegularTransfer {
                    hash: tx.hash.unwrap().encode_hex(),
                    block: tx.block_number.unwrap().to_string(),
                    from: tx.from.unwrap().encode_hex(),
                    to: tx.to.unwrap().encode_hex(),
                    value: tx.value.unwrap().encode_hex(),
                };

                app.regular_transfers.push(regular_transfer);
            }
        }
    }
}
