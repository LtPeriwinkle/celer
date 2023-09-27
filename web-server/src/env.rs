//! Server environment settings
use std::env;
use tracing::Level;

pub struct Environment {
    /// If server is running in debug mode
    pub logging_level: Level,
    /// If ANSI formatting is enabled in logs
    pub ansi: bool,
    /// Port to listen on
    pub port: u16,
    /// Directory to serve docs
    pub docs_dir: String,
    /// Directory to serve web client
    pub app_dir: String,
}

impl Environment {
    /// Parse environment from command line arguments and environment variables
    pub fn parse() -> Self {
        let mut logging_level = Level::INFO;
        let mut port = 8173;
        let mut ansi = true;

        if let Ok(x) = env::var("CELERSERVER_LOG") {
            match x.to_uppercase().as_ref() {
                "ERROR" => logging_level = Level::ERROR,
                "WARN" => logging_level = Level::WARN,
                "DEBUG" => logging_level = Level::DEBUG,
                "INFO" => logging_level = Level::INFO,
                _ => {
                    eprintln!("Invalid CELERSERVER_LOG value: {}", x);
                    eprintln!("Valid values: ERROR, WARN, DEBUG, INFO");
                    eprintln!("Defaulting to INFO");
                }
            }
        }

        if let Ok(x) = env::var("CELERSERVER_PORT") {
            match x.parse::<u16>() {
                Ok(x) => port = x,
                Err(_) => {
                    eprintln!("Invalid CELERSERVER_PORT value: {}", x);
                    eprintln!("Defaulting to 8173");
                }
            }
        }

        if let Ok(x) = env::var("CELERSERVER_ANSI") {
            if x == "0" {
                ansi = false;
            }
        }

        let docs_dir = if let Ok(x) = env::var("CELERSERVER_DOCS_DIR") {
            x
        } else {
            panic!("CELERSERVER_DOCS_DIR not set");
        };

        let app_dir = if let Ok(x) = env::var("CELERSERVER_APP_DIR") {
            x
        } else {
            panic!("CELERSERVER_APP_DIR not set");
        };

        for arg in env::args() {
            if arg == "--debug" {
                logging_level = Level::DEBUG;
            }
        }

        Self {
            logging_level,
            ansi,
            port,
            docs_dir,
            app_dir,
        }
    }
}
