use log::debug;
use env_logger;

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);
}

fn main() {
    let _ = env_logger::init();

    std::env::set_var("RUST_LOG", "debug");

    execute_query("DROP TABLE students");
}
