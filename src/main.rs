use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self};
use std::fs::File;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn find_matches(content: impl Iterator<Item = String>, pattern: &str, mut writer: impl std::io::Write) {
    for line in content {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path).with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    let reader = BufReader::new(f);
    let content = reader.lines().map(|l| l.expect("could not parse line"));

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    find_matches(content, &args.pattern, &mut handle);

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let content = vec![String::from("lorem ipsum"), String::from("dolor sit amet")];
    find_matches(content.into_iter(), "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
