use std::{fs, process::Command};

use clap::Parser;
use convert_case::Casing;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  problem: String,
}

fn main() {
  let args = Args::parse();
  let cargo_name: String = args
    .problem
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .to_case(convert_case::Case::Snake);
  let problem = args
    .problem
    .split_whitespace()
    .next()
    .expect("Get the problem number")
    .replace(".", "");
  let folder_name = args.problem.replace(".", "");

  fs::create_dir(folder_name.clone()).expect("Create the problem folder");
  Command::new("cmd")
    .args([
      "/C",
      &format!(
        "cd {} && cargo init --name {}-{}",
        folder_name, cargo_name, problem
      ),
    ])
    .output()
    .expect("Run cargo to init the project");
}
