use regex::Regex;

pub struct Hostname;

impl Hostname {
    pub fn get_hostname() -> String {
        match hostname::get() {
            Ok(name) => {
                let fqdn = name.to_string_lossy().to_string();
                let re = Regex::new(r"^[^.]+").unwrap();
                if let Some(captures) = re.captures(&fqdn) {
                    captures[0].to_string()
                } else {
                    "unknown-host".to_string()
                }
            }
            Err(_) => "unknown-host".to_string(),
        }
    }
}