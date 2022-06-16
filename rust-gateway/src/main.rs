use std::collections::HashMap;

pub mod http;

use std::time::Duration;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

/// This program demonstrates sending single message through a
/// `Producer`.  This is a convenient higher-level client that will
/// fit most use cases.
fn main() {
    env_logger::init();
    http::listen("0.0.0.0", 7777, &handle_connection);
}

fn handle_connection(req: http::Request) -> http::Response  {
    let broker = "kafka:9092";
    let topic = "test-topic";

    match req.body {
        Some(message) => 
            if let Err(e) = produce_message(message, topic, vec![broker.to_owned()]) {
                return http::Response { status: 500, protocol: "HTTP/1.1".to_string(), headers: HashMap::new(), body: Some(e.to_string()) };
            } else {
                return http::Response { status: 200, protocol: "HTTP/1.1".to_string(), headers: HashMap::new(), body: None };
            }
        None => 
            return http::Response { status: 400, protocol: "HTTP/1.1".to_string(), headers: HashMap::new(), body: Some("Missing body".to_string()) }
    }
}

fn produce_message<'a, 'b>(
    data: String,
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = Producer::from_hosts(brokers)
        // ~ give the brokers one second time to ack the message
        .with_ack_timeout(Duration::from_secs(1))
        // ~ require only one broker to ack the message
        .with_required_acks(RequiredAcks::One)
        // ~ build the producer with the above settings
        .create()?;

    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    producer.send(&Record::from_value(topic, data))?;

    Ok(())
}