fn execute_query(_query: &str) -> Result<(), &'static str>{
    Err("Sorry folks, park's closed.")
}

fn main() {

    log4rs::init_file("./log4rs.toml", Default::default()).unwrap();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}
