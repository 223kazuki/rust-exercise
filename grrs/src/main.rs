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

fn main() -> Result<(), ExitFailure> {

    let xs = vec![1, 2, 3];
    println!("The list is : {:?}", xs);

    eprintln!("This is error");

    let stdout = io::stdout();
    let mut handler = stdout.lock();
    writeln!(handler, "foo: {}", 42);

    let pb = ProgressBar::new(100);
    for i in 0..100 {
        std::thread::sleep_ms(10);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");


    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("Error reading `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
    // let args = Cli::from_args();
    // let result = std::fs::read_to_string("Cargo.toml");
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { return Err(error.into()); }
    // };
    // println!("file content: {}", content);
    // Ok(())
}
