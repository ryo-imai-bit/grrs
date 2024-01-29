use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        // 文字列リテラルや文字列は、一般的に文字列スライスとして関数に渡され、これによって、実際には所有権を渡す必要がないほとんど場合において柔軟性が増す。
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
