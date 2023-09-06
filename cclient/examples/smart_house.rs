use std::error::Error;

use cclient::CClient;
use cserver::models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let smart_house_client = CClient::new("http://127.0.0.1:8080")?;

    // add room
    println!("\nadding room...");
    let body = r#"{
        "name": "my-room33",
        "devices": [
            "device331",
            "device332"
        ]
    }"#;
    let body: models::AddRoom = serde_json::from_str(body).unwrap();
    let resp = smart_house_client
        .send_request::<models::AddRoom, models::AddedRoom>("/room", Some(body))
        .await?;
    println!("{:#?}", resp);

    // list rooms
    println!("\nlisting rooms...");
    let resp = smart_house_client
        .send_request::<&str, Vec<models::Room>>("/rooms", None)
        .await?;
    println!("{:#?}", resp);

    // add device
    println!("\nadding device...");
    let body = r#"{
        "room_id": "my-room2",
        "device_name": "device4"
    }"#;
    let body: models::AddDevice = serde_json::from_str(body).unwrap();
    let resp = smart_house_client
        .send_request::<models::AddDevice, Vec<models::Room>>("/device", Some(body))
        .await?;
    println!("{:#?}", resp);

    // ...

    Ok(())
}
