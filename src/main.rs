use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Write};
use std::fs::File;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path).with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    let reader = BufReader::new(f);
    let content = reader.lines().map(|l| l.expect("could not parse line"));

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for line in content {
        // 文字列リテラルや文字列は、一般的に文字列スライスとして関数に渡され、これによって、実際には所有権を渡す必要がないほとんど場合において柔軟性が増す。
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }

    Ok(())
}
