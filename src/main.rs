#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};
use ctrlc;
use logwatcher::{LogWatcher, LogWatcherAction, StartFrom};
use mysensors_logparser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn setup_args<'a>() -> ArgMatches<'a> {
    App::new("parse stream of MySensor logs")
        .version(crate_version!())
        .arg(
            Arg::with_name("logfile")
                .short("l")
                .long("logfile")
                .value_name("logfile to convert or watch")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("follow")
                .short("f")
                .long("follow")
                .value_name("follow logfile like 'tail -f <logfile>'"),
        )
        .arg(
            Arg::with_name("discover")
                .short("d")
                .long("discover")
                .value_name("discover"),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .value_name("timeout")
                .takes_value(true)
                .required(false),
        )
        .get_matches()
}

fn parse_file(filename: &str) -> Result<(), std::io::Error> {
    let filepath = Path::new(filename);
    let file = File::open(filepath)?;
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        //parse and print each line in the file
        println!("{}", mysensors_logparser::parse_log_line(&line?));
    }

    Ok(())
}

fn main() {
    // -l/--logfile -t/--timeout  -o/--output
    let cli = setup_args();

    ctrlc::set_handler(move || {
        println!("Ctrl-C detected, bye bye");
        std::process::exit(0) //exit OK (zero)
    })
    .expect("Error setting Ctrl-C handler");

    if let Some(logfile) = cli.value_of("logfile") {
        if let Some(_follow) = cli.value_of("follow") {
            // if following logfile, continuously watch for changes and parse
            let mut log_watcher = LogWatcher::register(logfile, StartFrom::Beginning).unwrap();
            log_watcher.watch(&mut move |_pos, _len, line: String| {
                println!("{}", mysensors_logparser::parse_log_line(&line));
                LogWatcherAction::None
            })
        } else {
            //just iterate across logfile and parse each line
            match parse_file(logfile) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
        }
    // } else if let Some(_discover) = cli.value_of("discover") {
    //     //get the serial ports and print out their info
    //     match serialport::available_ports() {
    //         Ok(infos) => print!(":?", infos),
    //         Err(e) => print!("error getting serial port infos: {}", e),
    //     }
    } else {
        println!("no logfile given, nothing to do!");
        println!("use --help to show how to use");
    }
}
