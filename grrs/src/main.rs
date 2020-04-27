use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self, Write};
use indicatif::ProgressBar;
use log::{info, warn};

#[derive(Debug)]
struct CustomError(String);

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[test]
fn check_answer_validity() {
    assert_eq!(1, 1);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

fn main() -> Result<(), ExitFailure> {

    // println!
    let xs = vec![1, 2, 3];
    println!("The list is : {:?}", xs);
    eprintln!("This is error");

    // writeln!
    let stdout = io::stdout();
    let mut handler = stdout.lock();
    writeln!(handler, "foo: {}", 42);

    // Progress bar
    let pb = ProgressBar::new(100);
    for i in 0..10 {
        std::thread::sleep_ms(10);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    // logging
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    // args
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
    // let path = args.path;
    // let content = std::fs::read_to_string(path)
    //     .with_context(|_| format!("Error reading `{}`", "file"))?;
    // println!("file content: {}", content);
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    // Ok(())
    // Let args = Cli::from_args();
    // let result = std::fs::read_to_string("Cargo.toml");
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { return Err(error.into()); }
    // };
    // println!("file content: {}", content);
    // Ok(())
}
