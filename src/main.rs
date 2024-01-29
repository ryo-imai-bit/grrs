use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let f = File::open(&args.path).unwrap();
    let reader = BufReader::new(f);
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let content = reader.lines().map(|l| l.expect("could not parse line"));

    for line in content {
        // 文字列リテラルや文字列は、一般的に文字列スライスとして関数に渡され、これによって、実際には所有権を渡す必要がないほとんど場合において柔軟性が増す。
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
