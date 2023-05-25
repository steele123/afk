use std::time::Duration;
use futures_util::StreamExt;
use shaco::model::ws::LcuSubscriptionType::JsonApiEvent;
use shaco::rest::RESTClient;
use shaco::ws::LcuWebsocketClient;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
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

        ws.subscribe(JsonApiEvent("/lol-champ-select/v1/session".to_string()))
            .await
            .unwrap();

        while let Some(msg) = ws.next().await {
            if msg.subscription_type.to_string() == "/lol-gameflow/v1/gameflow-phase" {
                let state = msg.data.as_str().unwrap();
                println!("Gameflow State: {}", state);
                if state != "ChampSelect" {
                    continue;
                }

                println!("Champ select started");
                client
                    .post("/lol-matchmaking/v1/ready-check/accept".to_string(), "{}")
                    .await
                    .unwrap();
                return;
            }


        }
    }
}
