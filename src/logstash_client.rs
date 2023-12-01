use std::collections::HashMap;

pub struct HttpClient {
    base_url: Option<String>,
}

impl HttpClient {
    pub fn new() -> Self {
        Self { base_url: None }
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = Some(base_url.to_string());
    }

    pub async fn make_request(&self, path: &str) -> Result<HashMap<String, serde_json::Value>, reqwest::Error> {
        let url = self.build_url(path);
        let resp = reqwest::get(&url).await?.json::<HashMap<String, serde_json::Value>>().await?;
        Ok(resp)
    }

    pub async fn get_logstash_cpu_percentage(&self) -> Result<u64, reqwest::Error> {
        let data = self.make_request("/_node/stats/process?pretty").await?;
    
        match data["process"]["cpu"]["percent"].as_u64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_load_average_1m(&self) -> Result<f64, reqwest::Error> {
        let data = self.make_request("/_node/stats/process?pretty").await?;
    
        match data["process"]["cpu"]["load_average"]["1m"].as_f64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_load_average_5m(&self) -> Result<f64, reqwest::Error> {
        let data = self.make_request("/_node/stats/process?pretty").await?;
    
        match data["process"]["cpu"]["load_average"]["5m"].as_f64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_load_average_15m(&self) -> Result<f64, reqwest::Error> {
        let data = self.make_request("/_node/stats/process?pretty").await?;
    
        match data["process"]["cpu"]["load_average"]["15m"].as_f64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_jvm_heap(&self) -> Result<u64, reqwest::Error> {
        let data = self.make_request("/_node/stats/jvm?pretty").await?;
    
        match data["jvm"]["mem"]["heap_used_percent"].as_u64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_memory(&self) -> Result<u64, reqwest::Error> {
        let data = self.make_request("/_node/stats/jvm?pretty").await?;
    
        match data["process"]["mem"]["total_virtual_in_bytes"].as_u64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_events_count(&self, pipeline: &str) -> Result<u64, reqwest::Error> {
        let url = if !pipeline.is_empty() {
            format!("/_node/stats/pipelines/{}?pretty", pipeline)
        } else {
            String::from("/_node/stats/events?pretty")
        };
    
        let data = self.make_request(&url).await?;
    
        let events_count = if !pipeline.is_empty() {
            data["pipelines"][pipeline]["events"]["in"].as_u64()
        } else {
            data["events"]["in"].as_u64()
        };
    
        match events_count {
            Some(value) => Ok(value),
            None => todo!(),
        }
    }

    pub async fn get_logstash_queue_events(&self, pipeline: &str) -> Result<u64, reqwest::Error> {
        let url = format!("/_node/stats/pipelines/{}?pretty", pipeline);
        let data = self.make_request(&url).await?;
    
        match data["pipelines"][pipeline]["queue"]["events_count"].as_u64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    pub async fn get_logstash_queue_size_in_bytes(&self, pipeline: &str) -> Result<u64, reqwest::Error> {
        let url = format!("/_node/stats/pipelines/{}?pretty", pipeline);
        let data = self.make_request(&url).await?;
    
        match data["pipelines"][pipeline]["queue"]["queue_size_in_bytes"].as_u64() {
            Some(value) => Ok(value),
            None => todo!()
        }
    }

    fn build_url(&self, path: &str) -> String {
        if let Some(ref base_url) = self.base_url {
            format!("{}{}", base_url, path)
        } else {
            panic!("Base URL not set");
        }
    }
}
