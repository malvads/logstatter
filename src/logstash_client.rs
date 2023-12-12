use serde_json::Value;
use std::collections::HashMap;
use reqwest::Error as ReqwestError;

#[derive(Clone)]
pub struct HttpClient {
    base_url: Option<String>,
}

#[derive(Debug)]
pub struct LogstashOperationResult {
    pub result_data: String,
    pub unit: String,
    pub valid: bool,
}

impl HttpClient {
    pub fn new() -> Self {
        Self { base_url: None }
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = Some(base_url.to_string());
    }

    pub async fn make_request(&self, path: &str) -> Result<HashMap<String, Value>, ReqwestError> {
        let url = self.build_url(path);
        let resp = reqwest::get(&url).await?.json::<HashMap<String, Value>>().await?;
        Ok(resp)
    }

    pub async fn get_logstash_cpu_percentage(&self) -> Result<u64, ReqwestError> {
        let data = self.make_request("/_node/stats/process?pretty").await?;
    
        data["process"]["cpu"]["percent"].as_u64().ok_or_else(|| todo!())
    }

    pub async fn get_logstash_load_average(&self, interval: &str) -> Result<f64, ReqwestError> {
        let data = self.make_request("/_node/stats/process?pretty").await?;
    
        data["process"]["cpu"]["load_average"][interval].as_f64().ok_or_else(|| todo!())
    }

    pub async fn get_logstash_jvm_heap(&self) -> Result<u64, ReqwestError> {
        let data = self.make_request("/_node/stats/jvm?pretty").await?;

        Ok(data.get("jvm")
            .and_then(|jvm| jvm.get("mem"))
            .and_then(|mem| mem.get("heap_used_percent"))
            .and_then(Value::as_u64)
            .unwrap_or(0))
    }
    
    pub async fn get_logstash_memory(&self) -> Result<u64, ReqwestError> {
        let data = self.make_request("/_node/stats/jvm?pretty").await?;
    
        Ok(data.get("process")
            .and_then(|process| process.get("mem"))
            .and_then(|mem| mem.get("total_virtual_in_bytes"))
            .and_then(Value::as_u64)
            .unwrap_or(0))
    }

    pub async fn get_logstash_events_count(&self, pipeline: &str) -> Result<u64, ReqwestError> {
        let url = if !pipeline.is_empty() {
            format!("/_node/stats/pipelines/{}?pretty", pipeline)
        } else {
            String::from("/_node/stats/events?pretty")
        };
    
        let data = self.make_request(&url).await?;
    
        let events_count = if !pipeline.is_empty() {
            data.get("pipelines")
                .and_then(|pipelines| pipelines.get(pipeline))
                .and_then(|pipeline| pipeline.get("events"))
                .and_then(|events| events.get("in"))
                .and_then(Value::as_u64)
        } else {
            data.get("events")
                .and_then(|events| events.get("in"))
                .and_then(Value::as_u64)
        };
    
        if let Some(count) = events_count {
            Ok(count)
        } else {
            let total_virtual_bytes = data
                .get("process")
                .and_then(|process| process.get("mem"))
                .and_then(|mem| mem.get("total_virtual_in_bytes"))
                .and_then(Value::as_u64)
                .unwrap_or(0);
    
            Ok(total_virtual_bytes)
        }
    }

    pub async fn get_logstash_queue_events(&self, pipeline: &str) -> Result<u64, ReqwestError> {
        let url = format!("/_node/stats/pipelines/{}?pretty", pipeline);
        let data = self.make_request(&url).await?;
    
        Ok(data
            .get("pipelines")
            .and_then(|pipelines| pipelines.get(pipeline))
            .and_then(|pipeline| pipeline.get("queue"))
            .and_then(|queue| queue.get("events_count"))
            .and_then(Value::as_u64)
            .unwrap_or(0))
    }
    
    pub async fn get_logstash_queue_size_in_bytes(&self, pipeline: &str) -> Result<u64, ReqwestError> {
        let url = format!("/_node/stats/pipelines/{}?pretty", pipeline);
        let data = self.make_request(&url).await?;
    
        Ok(data
            .get("pipelines")
            .and_then(|pipelines| pipelines.get(pipeline))
            .and_then(|pipeline| pipeline.get("queue"))
            .and_then(|queue| queue.get("queue_size_in_bytes"))
            .and_then(Value::as_u64)
            .unwrap_or(0))
    }

    fn build_url(&self, path: &str) -> String {
        self.base_url
            .as_ref()
            .map(|base_url| format!("{}{}", base_url, path))
            .unwrap_or_else(|| panic!("Base URL not set"))
    }

    pub async fn handle_logstash_operation(
        &self,
        monitor: &str,
        pipeline: &str,
    ) -> Result<LogstashOperationResult, reqwest::Error> {
        match monitor {
            "logstash_cpu" => {
                let result_data = self.get_logstash_cpu_percentage().await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "%".to_string(),
                    valid: pipeline.is_empty()
                })
            }
            "logstash_load_1" => {
                let result_data = self.get_logstash_load_average("1m").await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "%".to_string(),
                    valid: pipeline.is_empty()
                })
            }
            "logstash_load_5" => {
                let result_data = self.get_logstash_load_average("5m").await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "%".to_string(),
                    valid: pipeline.is_empty()
                })
            }
            "logstash_load_15" => {
                let result_data = self.get_logstash_load_average("15m").await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "%".to_string(),
                    valid: pipeline.is_empty()
                })
            }
            "logstash_heap" => {
                let result_data = self.get_logstash_jvm_heap().await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "%".to_string(),
                    valid: pipeline.is_empty()
                })
            }
            "logstash_memory" => {
                let result_data = self.get_logstash_memory().await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "bytes".to_string(),
                    valid: pipeline.is_empty()
                })
            }
            "logstash_events_per_pipeline" => {
                let result_data = self.get_logstash_events_count(pipeline).await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "event".to_string(),
                    valid: true
                })
            }
            "logstash_events_count_queue" => {
                let result_data = self.get_logstash_queue_events(pipeline).await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "event".to_string(),
                    valid: !pipeline.is_empty()
                })
            }
            "logstash_events_count_queue_bytes" => {
                let result_data = self.get_logstash_queue_size_in_bytes(pipeline).await?;
                Ok(LogstashOperationResult {
                    result_data: result_data.to_string(),
                    unit: "bytes".to_string(),
                    valid: !pipeline.is_empty()
                })
            }
            _ => {
    
                Ok(LogstashOperationResult {
                    result_data: "Unknown monitor type".to_string(),
                    unit: "unknow".to_string(),
                    valid: false
                })
            }
        }
    }
}
