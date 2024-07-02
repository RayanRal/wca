use std::fs;
use std::io::{self, IsTerminal, Stdin};
use clap::Parser;
use log::{error, LevelFilter};
use env_logger::Builder;
use wca::run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    chars: bool,

    #[arg(short)]
    words: bool,

    #[arg(short)]
    lines: bool,

    filepath: Option<String>,
}


fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let cli = Cli::parse();
    let stdin = io::stdin();
    println!("chars: {:?}", cli.chars);
    println!("words: {:?}", cli.words);
    println!("lines: {:?}", cli.lines);
    println!("filepath: {:?}", cli.filepath);
    let content = get_content(cli.filepath, stdin);
    if content.is_none() {
        error!("No content provided. Terminating")
    }
    run(cli.chars, cli.lines, cli.words, content.unwrap())
}

fn get_content(filepath_param: Option<String>, stdin: Stdin) -> Option<String> {
    match filepath_param {
        None => {
            if !stdin.is_terminal() {
                Some(io::read_to_string(stdin).expect("Can not read stdin"))
            } else {
                None
            }
        }
        Some(filepath) => {
            fs::read_to_string(filepath).ok()
        }
    }
}


