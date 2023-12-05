use clap::{App, Arg};

#[derive(Debug)]
pub struct CliParser {
    pub config_file: String,
}

impl CliParser {
    pub fn new() -> CliParser {
        let app = App::new("logstatter");

        let matches = app
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("file")
                    .help("Sets a custom config file")
                    .takes_value(true),
            )
            .get_matches();

        let config_file = matches.value_of("config").unwrap_or("config.yml").to_string();

        CliParser {
            config_file,
        }
    }
}
