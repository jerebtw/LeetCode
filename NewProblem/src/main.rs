use std::{fs, process::Command};

use clap::Parser;
use convert_case::Casing;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  name: String,
}

fn main() {
  let args = Args::parse();
  let cargo_name: String = args
    .name
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .replace("(", "")
    .replace(")", "")
    .to_case(convert_case::Case::Snake);
  let problem = args
    .name
    .split_whitespace()
    .next()
    .expect("Get the problem number")
    .replace(".", "");
  let folder_path = format!(
    "{}{}",
    "C:/Users/Jere/Desktop/Software/GitHub/LeetCode/",
    args.name.replace(".", "")
  );

  fs::create_dir(folder_path.clone()).expect("Create the problem folder");
  Command::new("cmd")
    .args([
      "/C",
      &format!(
        "cd {} && cargo init --name {}-{}",
        folder_path, cargo_name, problem
      ),
    ])
    .output()
    .expect("Run cargo to init the project");
  Command::new("cmd")
    .args(["/C", &format!("cd {} && code .", folder_path)])
    .output()
    .expect("Open vscode");
}
