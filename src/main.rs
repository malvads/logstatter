mod cli;
mod kafka;
mod config_manager;
mod logstash_client;
mod host;

use tokio::task;
use cli::CliParser;
use kafka::{KafkaProducer, MessageProducer};
use logstash_client::{HttpClient};
use config_manager::ConfigManager;
use host::Hostname;
use std::time::Duration;
use tokio::time::interval;

struct MonitorProcessor {
    monitor: String,
    pipeline: String,
}

impl MonitorProcessor {
    async fn process(&self, event: &mut MessageProducer, logstash_client: &mut HttpClient, producer: &mut KafkaProducer, topic: &str, hostname: &str) {
        match logstash_client.handle_logstash_operation(&self.monitor, &self.pipeline).await {
            Ok(logstash_result) => {

                event.set_timestamp();
                event.set_monitor(&self.monitor);
                event.set_value(&logstash_result.result_data);
                event.set_sensor_name(&hostname, &self.pipeline);
                event.set_unit(&logstash_result.unit);
                event.set_type("system");

                let result = event.get_object_as_str();

                if logstash_result.valid {
                    println!("{}", result);

                    match producer.produce_message(&topic, &result) {
                        Ok(_) => println!("Message sent successfully"),
                        Err(err) => eprintln!("Error producing message: {}", err),
                    }
                }
            }
            Err(err) => {
                eprintln!("Error handling Logstash operation: {}", err);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options = CliParser::new();
    let hostname = Hostname::get_hostname();
    let mut logstash_client = HttpClient::new();
    let event = MessageProducer::new();

    match ConfigManager::read_config(&cli_options.config_file) {
        Ok(mut config) => {
            let topic = &config.topic;
            logstash_client.set_base_url(&config.base_url);

            config.pipelines.push(String::new());

            let interval_duration = Duration::from_secs(60);
            let mut interval = interval(interval_duration);

            loop {
                interval.tick().await;
            
                let mut tasks = Vec::new();
            
                for monitor in &config.monitors {
                    for pipeline in &config.pipelines {
                        let monitor_clone = monitor.clone();
                        let pipeline_clone = pipeline.clone();
                        let mut event_clone = event.clone();
                        let mut logstash_client_clone = logstash_client.clone();
                        let mut producer_clone = KafkaProducer::new(config.brokers.clone());
                        let topic_clone = topic.clone();
                        let hostname_clone = hostname.clone();
            
                        let task = task::spawn(async move {
                            let monitor_processor = MonitorProcessor {
                                monitor: monitor_clone,
                                pipeline: pipeline_clone,
                            };
            
                            monitor_processor
                                .process(
                                    &mut event_clone,
                                    &mut logstash_client_clone,
                                    &mut producer_clone,
                                    &topic_clone,
                                    &hostname_clone,
                                )
                                .await;
                        });
            
                        tasks.push(task);
                    }
                }
            
                for task in tasks {
                    task.await.unwrap();
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading config: {}", err);
        }
    }

    Ok(())
}