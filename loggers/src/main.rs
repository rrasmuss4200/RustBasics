// use log::LevelFilter;
// use log4rs::append::file::FileAppender;
// use log4rs::append::rolling_file::RollingFileAppender;
// use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
// use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
// use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
// use log4rs::config::{Appender, Config, Root};
// use log4rs::encode::pattern::PatternEncoder;
// use log4rs::filter::threshold::ThresholdFilter;

// fn execute_query(_query: &str) -> Result<(), &'static str> {
//     Err("Sorry folks, park's closed.")
// }

// fn main() {
//     // Initialize log4rs from a file (optional, based on your existing setup)
//     log4rs::init_file("./log4rs.toml", Default::default()).unwrap();

//     // Log the execution result of the query
//     let response = execute_query("DROP TABLE students");
//     if let Err(err) = response {
//         log::error!("Failed to execute query: {}", err);
//     }

//     // Define the encoder
//     let pattern_encoder = PatternEncoder::new("{d} {l}::{m}{n}");

//     // Define the window size and roller
//     let window_size = 3; // log0, log1, log2
//     let fixed_window_roller = FixedWindowRoller::builder()
//         .build("log{}", window_size)
//         .unwrap();

//     // Define the size limit and trigger
//     let size_limit = 5 * 1024; // 5KB as max log file size to roll
//     let size_trigger = SizeTrigger::new(size_limit);

//     // Define the compound policy
//     let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

//     // Build the RollingFileAppender
//     let rolling_file_appender = RollingFileAppender::builder()
//         .encoder(Box::new(pattern_encoder))
//         .build("logfile", Box::new(compound_policy))
//         .unwrap();

//     // Configure log4rs
//     let config = Config::builder()
//         .appender(
//             Appender::builder()
//                 .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
//                 .build("logfile", Box::new(rolling_file_appender)),
//         )
//         .build(
//             Root::builder()
//                 .appender("logfile")
//                 .build(LevelFilter::Debug),
//         )
//         .unwrap();

//     // Initialize log4rs with the config
//     log4rs::init_config(config).unwrap();
// }

use log::{error, info};
use log4rs;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();

    // Log some messages
    info!("This is an info message.");
    error!("This is an error message.");
}


























