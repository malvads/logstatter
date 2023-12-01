mod cli;
mod config_manager;
mod logstash_client;

use cli::CliParser;
use config_manager::AppConfig;
use logstash_client::HttpClient;
use config_manager::ConfigManager;

async fn handle_logstash_operation(logstash_client: &HttpClient, option: &str, pipeline: &str) {
    match option {
        "c" => handle_async_operation(logstash_client.get_logstash_cpu_percentage().await),
        "l" => handle_async_operation(logstash_client.get_logstash_load_average_1m().await),
        "m" => handle_async_operation(logstash_client.get_logstash_load_average_5m().await),
        "n" => handle_async_operation(logstash_client.get_logstash_load_average_15m().await),
        "u" => handle_async_operation(logstash_client.get_logstash_jvm_heap().await),
        "v" => handle_async_operation(logstash_client.get_logstash_memory().await),
        "e" => handle_async_operation(logstash_client.get_logstash_events_count(pipeline).await),
        "w" => handle_async_operation(logstash_client.get_logstash_queue_events(pipeline).await),
        "z" => handle_async_operation(logstash_client.get_logstash_queue_size_in_bytes(pipeline).await),
        _ => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options = cli::CliParser::new();
    let mut logstash_client = HttpClient::new();

    match try_read_config(&cli_options.config_file) {
        Ok(config) => {
            logstash_client.set_base_url(&config.base_url);
            process_options(&logstash_client, &cli_options).await;
        }
        Err(err) => {
            eprintln!("Error reading config: {}", err);
            eprintln!("Please specify config...");
        }
    }

    Ok(())
}

fn try_read_config(config_file: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    ConfigManager::read_config(config_file).or_else(|_| {
        ConfigManager::read_config("/etc/logstastter.conf")
    })
}

async fn process_options(logstash_client: &HttpClient, cli_options: &CliParser) {
    for &option in &cli_options.options {
        if cli_options.is_option_passed(option) {
            handle_logstash_operation(
                logstash_client,
                option,
                cli_options.get_option_value(option).as_str(),
            )
            .await;
        }
    }
}

fn handle_async_operation<T>(result: Result<T, reqwest::Error>)
where
    T: std::fmt::Debug,
{
    match result {
        Ok(value) => println!("{:?}", value),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}