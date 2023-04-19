use kafka::producer::{Producer, Record};

pub struct MyProducer {
    producer: Producer,
}

impl MyProducer {
    pub fn new(hosts: Vec<String>) -> Self {
        let producer = Producer::from_hosts(hosts).create().unwrap();

        Self { producer: producer }
    }

    pub fn send_data_to_topic(&mut self, topic: &str, data: String) {
        let record = Record::from_value(topic, data.as_bytes());
        self.producer.send(&record).unwrap();
    }
}
