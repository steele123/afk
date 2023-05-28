mod config;

use std::time::Duration;
use futures_util::StreamExt;
use shaco::model::ws::LcuSubscriptionType::JsonApiEvent;
use shaco::rest::RESTClient;
use shaco::ws::LcuWebsocketClient;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Made by steele#7375, the code is available @ github.com/steele123/afk");
    println!("Go afk I'll do the rest :)");
    println!("Trying to connect to League Client...");

    let mut connected = false;
    loop {
        let client = match RESTClient::new() {
            Ok(client) => {
                connected = true;
                client
            }
            Err(_) => {
                if connected {
                    println!("Disconnected from League Client");
                    connected = false;
                }

                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        let mut ws = match LcuWebsocketClient::connect().await {
            Ok(ws) => ws,
            Err(_) => {
                sleep(Duration::from_secs(2)).await;
                LcuWebsocketClient::connect().await.unwrap()
            }
        };

        println!("Connected to League Client!");

        ws
            .subscribe(JsonApiEvent("/lol-gameflow/v1/gameflow-phase".to_string()))
            .await
            .unwrap();

        ws
            .subscribe(JsonApiEvent("/lol-champ-select/v1/ongoing-swap".to_string()))
            .await
            .unwrap();

        while let Some(msg) = ws.next().await {
            if msg.subscription_type.to_string() == "OnJsonApiEvent_lol-gameflow_v1_gameflow-phase" {
                let state = msg.data.as_str().unwrap();
                println!("Gameflow State: {}", state);
                if state != "ReadyCheck" {
                    continue;
                }

                println!("Ready check found, accepting...");

                // Delay 1 second to make sure the ready check is fully loaded
                sleep(Duration::from_secs(1)).await;

                let _ = client
                    .post("/lol-matchmaking/v1/ready-check/accept".to_string(), "")
                    .await;

                return;
            }

            /*
            if msg.subscription_type.to_string() == "OnJsonApiEvent_lol-champ-select_v1_ongoing-swap" {
                if msg.data.as_object().unwrap().get("state").unwrap().as_str().unwrap() != "RECEIVED" {
                    continue;
                }

                println!("Ongoing swap found, accepting...");


                return;
            }*/
        }
    }
}
