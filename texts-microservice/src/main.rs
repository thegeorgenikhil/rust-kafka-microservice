mod my_consumer;
mod my_producer;
mod texts;

use my_consumer::MyConsumer;
use my_producer::MyProducer;
use texts::Texts;

fn main() {
    let hosts = vec!["localhost:9092".to_string()];

    let mut texts = Texts::new();
    let mut consumer = MyConsumer::new(hosts.clone(), "actions".to_string());
    let mut producer = MyProducer::new(hosts);
    // put here to show that the microservice has started
    println!("Started...");

    loop {
        for ms in consumer.consume_events().iter() {
            for m in ms.messages() {
                // when the consumer receives an event, this block is executed
                let event_data = MyConsumer::get_event_data(m);
                let action = event_data["action"].to_string();

                if action == "\"add\"" {
                    texts.add_text(event_data["value"].to_string());
                } else if action == "\"remove\"" {
                    let index = event_data["value"].to_string().parse::<usize>().unwrap();
                    texts.remove_text(index);
                } else {
                    println!("Invalid action");
                }

                producer.send_data_to_topic("texts", texts.to_json());
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed();
    }
}
