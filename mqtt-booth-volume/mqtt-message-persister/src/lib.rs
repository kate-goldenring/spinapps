use chrono::{DateTime, Utc};
use spin_mqtt_sdk::{mqtt_component, Metadata, Payload};
use spin_sdk::sqlite::{Connection, Value};
use spin_sdk::variables;
use serde::Deserialize;

const DEFAULT_THRESHOLD: &str = "100";

#[derive(Deserialize)]
struct Data {
    volume: i64,
}

#[mqtt_component]
async fn handle_message(message: Payload, metadata: Metadata) -> anyhow::Result<()> {
    let message = String::from_utf8_lossy(&message);
    println!(
        "Message received by wasm component: '{}' on topic '{}'",
        message,
        metadata.topic
    );
    let data = serde_json::from_str::<Data>(&message)?; 
    let threshold = variables::get("threshold").unwrap_or(DEFAULT_THRESHOLD.to_string()).parse::<i64>().unwrap();
    if data.volume > threshold {
        let datetime: DateTime<Utc> = std::time::SystemTime::now().into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        let connection = Connection::open_default()?;

        let execute_params = [
            Value::Text(metadata.topic),
            Value::Integer(data.volume),
            Value::Text(formatted_time),
        ];
        connection.execute(
            "INSERT INTO noise_log (source, volume, timestamp) VALUES (?, ?, ?)",
            execute_params.as_slice(),
        )?;
    }
    Ok(())
}