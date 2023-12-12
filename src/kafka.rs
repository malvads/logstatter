use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;
use serde_json::{json, Map, Value};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct MessageProducer {
    fields: Map<String, Value>,
}

pub struct KafkaProducer {
    producer: Producer,
}

impl KafkaProducer {
    pub fn new(brokers: Vec<String>) -> Self {
        let producer = Producer::from_hosts(brokers)
            .with_ack_timeout(std::time::Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .with_connection_idle_timeout(std::time::Duration::from_secs(30))
            .create()
            .expect("Producer creation error");

        KafkaProducer { producer }
    }

    pub fn produce_message(&mut self, topic: &str, message: &str) -> Result<(), KafkaError> {
        let record = Record::from_value(topic, message);
        self.producer.send(&record)?;
        Ok(())
    }
}


impl MessageProducer {
    pub fn new() -> Self {
        let field_map = Map::new();
        Self { fields: field_map }
    }

    pub fn set_timestamp(&mut self) {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        self.fields.insert("timestamp".to_string(), json!(timestamp));
    }

    pub fn set_monitor(&mut self, monitor: &str) {
        self.fields.insert("monitor".to_string(), json!(monitor));
    }

    pub fn set_value(&mut self, value: &str) {
        self.fields.insert("value".to_string(), json!(value));
    }

    pub fn set_sensor_name(&mut self, sensor_name: &str, pipeline: &str) {
        let full_sensor_name = if !pipeline.is_empty() {
            format!("{}-{}", sensor_name, pipeline)
        } else {
            sensor_name.to_string()
        };
    
        self.fields.insert("sensor_name".to_string(), json!(full_sensor_name));
    }    

    pub fn set_type(&mut self, type_value: &str) {
        self.fields.insert("type".to_string(), json!(type_value));
    }

    pub fn set_unit(&mut self, unit: &str) {
        self.fields.insert("unit".to_string(), json!(unit));
    }

    pub fn get_object_as_str(&self) -> String {
        serde_json::to_string(&self.fields).unwrap_or_else(|_| String::new())
    }
}
