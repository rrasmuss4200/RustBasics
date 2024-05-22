use log::{error, info};
use log4rs;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();

    // Log some messages
    info!("This is an info message.");
    error!("This is an error message.");
}


























