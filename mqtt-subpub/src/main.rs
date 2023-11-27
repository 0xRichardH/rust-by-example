use std::time::Duration;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use tokio::{task, time};

const MQTT_BROKER_HOST: &str = "broker.emqx.io";
const MQTT_BROKER_PORT: u16 = 1883;
const MQTT_CLIENT_ID: &str = "93292c9e-ac16-45d8-9a73-7b68972058sd";
const MQTT_TOPIC: &str = "rust/mqtt";
const MQTT_KEEP_ALIVE_DURATION: Duration = Duration::from_secs(5);
const MQTT_ASYNC_CHANNEL_CAP: usize = 10;

#[tokio::main]
async fn main() -> Result<(), rumqttc::ClientError> {
    let mut mqtt_options = MqttOptions::new(MQTT_CLIENT_ID, MQTT_BROKER_HOST, MQTT_BROKER_PORT);
    mqtt_options.set_keep_alive(MQTT_KEEP_ALIVE_DURATION);

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, MQTT_ASYNC_CHANNEL_CAP);
    // subscriber
    client.subscribe(MQTT_TOPIC, QoS::AtMostOnce).await?;

    // publisher
    task::spawn(async move {
        for i in 0..5 {
            if let Err(e) = client
                .publish(
                    MQTT_TOPIC,
                    QoS::AtLeastOnce,
                    false,
                    format!("Hello World {}", i),
                )
                .await
            {
                eprintln!("Error sending message: {:?}", e);
            }
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    while let Ok(event) = eventloop.poll().await {
        println!("Received event: {:?}", event);
        if let Event::Incoming(Packet::Publish(msg)) = event {
            println!("==========================================");
            println!("Received subscribe message: {:?}", msg.payload);
            println!("==========================================");
        }
    }

    Ok(())
}
