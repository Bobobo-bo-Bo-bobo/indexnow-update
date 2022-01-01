use getopts::Options;
use log::{debug, error, info};
use std::env;
use std::process;

mod config;
mod constants;
mod logging;
mod scan;
mod sqlite3;
mod usage;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut log_level = log::LevelFilter::Info;
    let mut purge_old = false;

    options.optflag("D", "debug", "Enable debug output");
    options.optopt("c", "config", "Path to configuration file", "config_file");
    options.optflag("h", "help", "Show help text");
    options.optflag("p", "purge", "Remove stale database entries");
    options.optflag("v", "version", "Show version information");
    options.optflag("q", "--quiet", "Quiet operation");

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments ({})", e);
            process::exit(1);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(0);
    }

    if opts.opt_present("v") {
        usage::show_version();
        process::exit(0);
    }

    if opts.opt_present("q") {
        log_level = log::LevelFilter::Warn;
    }

    if opts.opt_present("D") {
        log_level = log::LevelFilter::Debug;
    }

    if opts.opt_present("p") {
        purge_old = true;
    };

    let config_file = match opts.opt_str("c") {
        Some(v) => v,
        None => {
            eprintln!("Error: Configuration file is mandatory");
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    // Initialise logging via fern
    match logging::init(log_level) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: Unable to initialise logging - {}", e);
            process::exit(1);
        }
    };

    if opts.free.is_empty() {
        eprintln!("Error: Missing path to HTML data is missing");
        println!();
        usage::show_usage();
        process::exit(1);
    }

    if opts.free.len() > 1 {
        eprintln!("Error: Only a single value for HTML data is allowed");
        println!();
        usage::show_usage();
        process::exit(1);
    }

    let mut html_dir: String = opts.free[0].clone();
    if !html_dir.ends_with('/') {
        html_dir = format!("{}/", html_dir);
    };

    debug!("HTML data directory is at {}", html_dir);
    info!("Parsing configuration file {}", config_file);
    let config = match config::parse_config_file(&config_file) {
        Ok(v) => v,
        Err(e) => {
            error!("Can't read configuration from {}: {}", config_file, e);
            process::exit(1);
        }
    };

    debug!("Parsed configuration: {:?}", config);
    let fext_list = match config.file_extensions {
        Some(v) => v,
        None => {
            error!("List of file extensions is empty");
            process::exit(1);
        }
    };

    debug!("Opening database connection to {}", config.database);
    let mut db_handle = match sqlite3::open(&config.database) {
        Ok(v) => v,
        Err(e) => {
            error!("Can't open databse file {}: {}", config.database, e);
            process::exit(1);
        }
    };

    let indexnow = match scan::build_update_list(&html_dir, &mut db_handle, fext_list, purge_old) {
        Ok(v) => v,
        Err(e) => {
            error!("Unable to build file list: {}", e);
            process::exit(1);
        }
    };
    println!("> {:?}", indexnow);
}
