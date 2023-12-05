pub struct Hostname;

impl Hostname {
    pub fn get_hostname() -> String {
        match hostname::get() {
            Ok(name) => name.to_string_lossy().to_string(),
            Err(_) => "unknown-host".to_string(),
        }
    }
}
