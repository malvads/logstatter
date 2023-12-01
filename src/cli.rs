use clap::{App, Arg};

#[derive(Debug)]
pub struct CliParser {
    pub options: Vec<&'static str>,
    pub config_file: String,
    pub parsed_options: Vec<(String, Option<String>)>,
}

impl CliParser {
    pub fn new() -> CliParser {
        let mut options = vec!["c", "l", "m", "n", "u", "v"];
        let required_options = vec!["e", "w", "z"];
        let mut app = App::new("Logstash API crawler");

        for option in options.iter().chain(required_options.iter()) {
            app = app.arg(
                Arg::with_name(option)
                    .short(option)
                    .long(option)
                    .takes_value(required_options.contains(option)),
            );
        }

        let matches = app
            .arg(
                Arg::with_name("config")
                    .short("a")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            )
            .get_matches();

        options.extend_from_slice(&required_options);

        let parsed_options: Vec<(String, Option<String>)> = options
            .iter()
            .map(|opt| {
                let value = matches.value_of(opt).map(|v| v.to_string());
                (opt.to_string(), value)
            })
            .collect();

        let config_file = matches.value_of("config").unwrap_or("config.yml").to_string();

        CliParser {
            options,
            config_file,
            parsed_options,
        }
    }

    pub fn is_option_passed(&self, option_flag: &str) -> bool {
        std::env::args().any(|arg| arg.trim_start_matches('-') == option_flag)
    }

    pub fn get_option_value(&self, option_flag: &str) -> String {
        self.parsed_options
            .iter()
            .find(|(opt, _)| opt == option_flag)
            .and_then(|(_, value)| value.clone())
            .unwrap_or_default()
    }
}
