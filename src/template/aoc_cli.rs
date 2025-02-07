/// Wrapper module around the "aoc-cli" command-line.
use std::{
    fmt::Display,
    process::{Command, Output, Stdio},
};

use crate::template::Day;

#[derive(Debug)]
pub enum AocCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
}

pub enum DownloadMode {
    InputOnly,
    PuzzleOnly,
    InputAndPuzzle,
}

impl DownloadMode {
    fn modify_args(&self, input_path: &str, puzzle_path: &str) -> Vec<String> {
        let mut args = vec!["--overwrite".into()];
        match self {
            DownloadMode::InputOnly => {
                args.extend(["--input-only", "--input-file", input_path].iter().map(|s| s.to_string()));
            }
            DownloadMode::PuzzleOnly => {
                args.extend(["--puzzle-only", "--puzzle-file", puzzle_path].iter().map(|s| s.to_string()));
            }
            DownloadMode::InputAndPuzzle => {
                args.extend([
                    "--input-file", input_path,
                    "--puzzle-file", puzzle_path
                ].iter().map(|s| s.to_string()));
            }
        }
        args
    }

    fn downloads_input(&self) -> bool {
        matches!(self, DownloadMode::InputOnly | DownloadMode::InputAndPuzzle)
    }

    fn downloads_puzzle(&self) -> bool {
        matches!(self, DownloadMode::PuzzleOnly | DownloadMode::InputAndPuzzle)
    }
}

impl Display for AocCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocCommandError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
            AocCommandError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
            AocCommandError::BadExitStatus(_) => {
                write!(f, "aoc-cli exited with a non-zero status.")
            }
        }
    }
}

pub fn check() -> Result<(), AocCommandError> {
    Command::new("aoc")
        .arg("-V")
        .output()
        .map_err(|_| AocCommandError::CommandNotFound)?;
    Ok(())
}

pub fn read(day: Day) -> Result<Output, AocCommandError> {
    let puzzle_path = get_puzzle_path(day);

    let args = build_args(
        "read",
        &[
            "--description-only".into(),
            "--puzzle-file".into(),
            puzzle_path,
        ],
        day,
    );

    call_aoc_cli(&args)
}

pub fn download(day: Day, download_variants: DownloadMode) -> Result<Output, AocCommandError> {
    let input_path = get_input_path(day);
    let puzzle_path = get_puzzle_path(day);

    let args = &build_args(
        "download",
        &download_variants.modify_args(&input_path, &puzzle_path)[..],
        day,
    );

    let output = call_aoc_cli(&args)?;
    println!("---");
    if download_variants.downloads_input() {
        println!("🎄 Successfully wrote input to \"{}\".", &input_path);
    }
    if download_variants.downloads_puzzle() {
        println!("🎄 Successfully wrote puzzle to \"{}\".", &puzzle_path);
    }
    Ok(output)
}

pub fn submit(day: Day, part: u8, result: &str) -> Result<Output, AocCommandError> {
    // workaround: the argument order is inverted for submit.
    let mut args = build_args("submit", &[], day);
    args.push(part.to_string());
    args.push(result.to_string());
    call_aoc_cli(&args)
}

fn get_input_path(day: Day) -> String {
    format!("data/inputs/{day}.txt")
}

fn get_puzzle_path(day: Day) -> String {
    format!("data/puzzles/{day}.md")
}

fn get_year() -> Option<u16> {
    match std::env::var("AOC_YEAR") {
        Ok(x) => x.parse().ok().or(None),
        Err(_) => None,
    }
}

fn build_args(command: &str, args: &[String], day: Day) -> Vec<String> {
    let mut cmd_args = args.to_vec();

    if let Some(year) = get_year() {
        cmd_args.push("--year".into());
        cmd_args.push(year.to_string());
    }

    cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);

    cmd_args
}

fn call_aoc_cli(args: &[String]) -> Result<Output, AocCommandError> {
    // println!("Calling >aoc with: {}", args.join(" "));
    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| AocCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AocCommandError::BadExitStatus(output))
    }
}
