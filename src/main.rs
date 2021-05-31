use {
    athanasia::Engine,
    clap::{crate_authors, crate_version, Clap},
    flexi_logger::Logger,
};

#[derive(Clap, Debug, Clone)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(", ")
)]
pub struct Config {
    input_file: String,
    /// Count of threads that should be used for processing, limited to 8.
    #[clap(short)]
    pub thread_count: Option<u8>,
    /// Possible values: info, debug, error, warn. Fallback: info.
    #[clap(short)]
    pub log_level: Option<String>,
}

fn initialize_config() -> Config {
    let mut config = Config::parse();
    config.thread_count = match config.thread_count {
        Some(v) => {
            if v > 8 {
                Some(8)
            } else {
                Some(v)
            }
        }
        None => Some(1),
    };
    config.log_level = match config.log_level {
        None => Some(format!("info")),
        Some(v) => {
            if !["info", "debug", "error", "warn"].contains(&&v[..]) {
                Some(format!("info"))
            } else {
                Some(v)
            }
        }
    };
    config
}

fn main() {
    let config = initialize_config();

    Logger::with_str(config.log_level.unwrap())
        .log_to_file()
        .start()
        .unwrap();

    Engine::new(config.thread_count.unwrap()).run(&config.input_file);
}
