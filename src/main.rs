use clap::Parser;
use ctrlc;
use logwatcher::{LogWatcher, LogWatcherAction, StartFrom};
use mysensors_logparser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// continue to follow the logfile (use ctrl-C to quit)
    #[clap(short, long)]
    follow: bool,

    ///when following, dump whole file, then start following
    #[clap(short, long)]
    beginning: bool,

    /// name of the logfile to format
    #[clap(short, long)]
    logfile: String,
}

fn parse_file(filename: &str) -> Result<(), std::io::Error> {
    let filepath = Path::new(filename);
    let file = File::open(filepath)?;
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        //parse and print each line in the file
        match line {
            Ok(l) => println!("{}", mysensors_logparser::parse_log_line(&l)),
            Err(e) => println!("error reading line: {}", e),
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    println!("{:#?}\n\n\n", cli);

    ctrlc::set_handler(move || {
        println!("\nCtrl-C detected, bye bye");
        std::process::exit(0) //exit OK (zero)
    })
    .expect("Error setting Ctrl-C handler");

    if cli.follow {
        // if following logfile, continuously watch for changes and parse
        let mut log_watcher;
        // either dump whole file or just start watching for changes
        if cli.beginning {
            log_watcher = LogWatcher::register(cli.logfile, StartFrom::Beginning).unwrap();
        } else {
            log_watcher = LogWatcher::register(cli.logfile, StartFrom::End).unwrap();
        }
        //actually watch for changes and parse/format each new line
        log_watcher.watch(&mut move |_pos, _len, line: String| {
            println!("{}", mysensors_logparser::parse_log_line(&line));
            LogWatcherAction::None
        })
    } else {
        //just iterate across logfile and parse each line
        match parse_file(&cli.logfile) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }
}
