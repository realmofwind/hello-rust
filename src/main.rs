use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: PathBuf,
}

fn main() {
  let args = Cli::parse();

  let file = File::open(&args.path).expect("could not read file");
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line.expect("could not read line");
    if line.contains(&args.pattern) {
      println!("{}", line);
    }
  }

}